//! Getters internals
use std::convert::TryFrom;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{
    Attribute, DataStruct, DeriveInput, Error, Fields, FieldsNamed, Ident, LitStr, Result, Type,
};

use crate::{extract::named_struct, faultmsg::Problem};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Action {
    Skip,
    Rename(Ident),
    Copy,
}

impl Parse for Action {
    fn parse(input: ParseStream) -> Result<Self> {
        syn::custom_keyword!(skip);
        syn::custom_keyword!(rename);
        syn::custom_keyword!(copy);

        if input.peek(skip) {
            let _ = input.parse::<skip>()?;
            if !input.is_empty() {
                Err(Error::new(Span::call_site(), Problem::TokensFollowSkip))
            } else {
                Ok(Action::Skip)
            }
        } else if input.peek(rename) {
            let _ = input.parse::<rename>()?;
            let _ = input.parse::<syn::Token![=]>()?;
            let name = input.parse::<LitStr>()?;
            if !input.is_empty() {
                Err(Error::new(Span::call_site(), Problem::TokensFollowNewName))
            } else {
                Ok(Action::Rename(Ident::new(
                    name.value().as_str(),
                    Span::call_site(),
                )))
            }
        } else if input.peek(copy) {
            let _ = input.parse::<copy>()?;
            if !input.is_empty() {
                Err(Error::new(Span::call_site(), Problem::TokensFollowCopy))
            } else {
                Ok(Action::Copy)
            }
        } else {
            Err(Error::new(Span::call_site(), Problem::InvalidAttribute))
        }
    }
}

#[derive(Debug, Clone)]
struct Doc(TokenStream);

#[derive(Debug, Clone)]
struct Work {
    // Whether to carry out an action (skip or rename) on the field.
    special: Option<Action>,

    // The documentation, if any, on the field.
    docs: Vec<Doc>,
}

impl TryFrom<&[Attribute]> for Work {
    type Error = Error;

    fn try_from(attributes: &[Attribute]) -> Result<Self> {
        let mut special: Option<Action> = None;
        let mut docs: Vec<Doc> = Vec::new();

        for attr in attributes {
            if attr.path().is_ident("getter") {
                special = Some(attr.parse_args::<Action>()?);
            }

            if attr.path().is_ident("doc") {
                docs.push(Doc(attr.to_token_stream()));
            }
        }

        Ok(Work { special, docs })
    }
}

pub enum ReturnKind {
    Copy,
    Reference,
}

impl Default for ReturnKind {
    fn default() -> Self {
        ReturnKind::Reference
    }
}

pub struct Field {
    ty: Type,
    name: Ident,
    getter: Ident,
    return_kind: ReturnKind,
    docs: Vec<Doc>,
}

impl Field {
    fn from_field(field: &syn::Field) -> Result<Option<Self>> {
        let name: Ident = field
            .ident
            .clone()
            .ok_or(Error::new(Span::call_site(), Problem::UnnamedField))?;

        let work = Work::try_from(field.attrs.as_slice())?;

        match work {
            Work {
                special: Some(Action::Skip),
                ..
            } => Ok(None),
            Work { special, docs } => {
                let ty = field.ty.clone();
                let getter = match &special {
                    Some(Action::Rename(ident)) => ident.clone(),
                    _ => name.clone(),
                };
                let return_kind = match &special {
                    Some(Action::Copy) => ReturnKind::Copy,
                    _ => {
                        #[cfg(feature = "auto_copy_getters")]
                        if type_implements_copy(&ty) {
                            ReturnKind::Copy
                        } else {
                            ReturnKind::Reference
                        }
                        #[cfg(not(feature = "auto_copy_getters"))]
                        ReturnKind::Reference
                    }
                };
                Ok(Some(Field {
                    ty,
                    name,
                    getter,
                    return_kind,
                    docs,
                }))
            }
        }
    }

    fn from_fields_named(fields_named: &FieldsNamed) -> Result<Vec<Self>> {
        fields_named
            .named
            .iter()
            .try_fold(Vec::new(), |mut fields, field| {
                if let Some(field) = Field::from_field(field)? {
                    fields.push(field);
                }

                Ok(fields)
            })
    }

    fn from_struct(structure: &DataStruct) -> Result<Vec<Self>> {
        let fields_named = match structure.fields {
            Fields::Named(ref fields) => Ok(fields),
            Fields::Unnamed(_) => Err(Error::new(Span::call_site(), Problem::UnnamedField)),
            Fields::Unit => Err(Error::new(Span::call_site(), Problem::UnitStruct)),
        }?;

        Self::from_fields_named(fields_named)
    }

    fn emit(&self, struct_name: &Ident) -> TokenStream {
        let returns = &self.ty;
        let field_name = &self.name;
        let getter_name = &self.getter;

        let doc_comments: Vec<TokenStream> = if self.docs.is_empty() {
            let comment = format!(
                " Get field `{}` from instance of `{}`.",
                field_name, struct_name,
            );

            vec![quote!(#[doc=#comment])]
        } else {
            self.docs.iter().map(|d| d.0.to_owned()).collect()
        };

        match &self.ty {
            Type::Reference(tr) => {
                let lifetime = tr.lifetime.as_ref();
                quote!(
                    #(#doc_comments)*
                    pub fn #getter_name(&#lifetime self) -> #returns {
                        self.#field_name
                    }
                )
            }
            _ => match self.return_kind {
                ReturnKind::Copy => quote!(
                    #(#doc_comments)*
                    pub fn #getter_name(&self) -> #returns {
                        self.#field_name
                    }
                ),
                ReturnKind::Reference => quote!(
                    #(#doc_comments)*
                    pub fn #getter_name(&self) -> &#returns {
                        &self.#field_name
                    }
                ),
            },
        }
    }
}

pub struct NamedStruct<'a> {
    original: &'a DeriveInput,
    name: Ident,
    fields: Vec<Field>,
}

impl<'a> NamedStruct<'a> {
    pub fn emit(&self) -> TokenStream {
        let (impl_generics, struct_generics, where_clause) =
            self.original.generics.split_for_impl();
        let struct_name = &self.name;
        let methods: Vec<TokenStream> = self
            .fields
            .iter()
            .map(|field| field.emit(&self.name))
            .collect();

        let impl_comment = " Auto-generated by `derive_getters::Getters`.";
        let impl_doc_comment = quote!(#[doc=#impl_comment]);

        quote!(
            #impl_doc_comment
            impl #impl_generics #struct_name #struct_generics
                #where_clause
            {
                #(#methods)*
            }
        )
    }
}

impl<'a> TryFrom<&'a DeriveInput> for NamedStruct<'a> {
    type Error = Error;

    fn try_from(node: &'a DeriveInput) -> Result<Self> {
        let struct_data = named_struct(node)?;
        let fields = Field::from_struct(struct_data)?;

        Ok(NamedStruct {
            original: node,
            name: node.ident.clone(),
            fields,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_action() -> Result<()> {
        let a: Action = syn::parse_str("skip")?;
        assert!(a == Action::Skip);

        let r: Result<Action> = syn::parse_str("skip = blah");
        assert!(r.is_err());

        let a: Action = syn::parse_str("rename = \"hello\"")?;
        let check = Action::Rename(Ident::new("hello", Span::call_site()));
        assert!(a == check);

        let r: Result<Action> = syn::parse_str("rename + \"chooga\"");
        assert!(r.is_err());

        let r: Result<Action> = syn::parse_str("rename = \"chooga\" | bongle");
        assert!(r.is_err());

        Ok(())
    }
}

#[cfg(feature = "auto_copy_getters")]
fn type_implements_copy(ty: &syn::Type) -> bool {
    match ty {
        Type::Array(array) => type_implements_copy(&array.elem), // Assuming array.elem is the type of the elements and we recursively check if it implements Copy
        Type::BareFn(_) => true,                                 // Function pointers implement Copy
        Type::Group(group) => type_implements_copy(&group.elem),
        Type::ImplTrait(_) => false, // ImplTrait does not implement Copy
        Type::Infer(_) => false,     // Infer does not implement Copy
        Type::Macro(_) => false,     // Macros do not implement Copy
        Type::Never(_) => true,      // The Never type (!) implements Copy
        Type::Paren(paren) => type_implements_copy(&paren.elem),
        Type::Path(path) => type_path_implements_copy(path),
        Type::Ptr(_) => false,       // Raw pointers do not implement Copy
        Type::Reference(_) => false, // References do not implement Copy
        Type::Slice(slice) => type_implements_copy(&slice.elem), // Similar to arrays
        Type::TraitObject(_) => false, // Trait objects do not implement Copy
        Type::Tuple(tuple) => tuple.elems.iter().all(type_implements_copy), // All elements in the tuple must implement Copy
        Type::Verbatim(_) => false, // Verbatim is a catch-all and does not implement Copy
        _ => false,                 // Catch all for any other types not explicitly matched
    }
}

#[cfg(feature = "auto_copy_getters")]
fn type_path_implements_copy(path: &syn::TypePath) -> bool {
    match path.to_token_stream().to_string().as_str() {
        "u8" | "u16" | "u32" | "u64" | "u128" | "i8" | "i16" | "i32" | "i64" | "i128" | "f32"
        | "f64" | "bool" | "char" | "usize" | "isize" => true,
        _ => false,
    }
}
