//! Getters internals
use std::convert::TryFrom;

use proc_macro2::{TokenStream, Span};
use quote::{quote, ToTokens};
use syn::{
    DeriveInput,
    FieldsNamed,
    Type,
    Ident,
    LitStr,
    Result,
    Error,
    Attribute,
    parse::{Parse, ParseStream},
};

use crate::{
    extract::{named_fields, named_struct},
    faultmsg::Problem,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Action {    
    Skip,
    Rename(Ident),
}

impl Parse for Action {
    fn parse(input: ParseStream) -> Result<Self> {
        syn::custom_keyword!(skip);
        syn::custom_keyword!(rename);
        
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
                Ok(Action::Rename(Ident::new(name.value().as_str(), Span::call_site())))
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
            if attr.path.is_ident("getter") {
                special = Some(attr.parse_args::<Action>()?);
            }

            if attr.path.is_ident("doc") {
                docs.push(Doc(attr.to_token_stream()));
            }
        }

        Ok(Work { special, docs })
    }
}

pub struct Field {
    ty: Type,    
    name: Ident,
    getter: Ident,
    docs: Vec<Doc>,
}

impl Field {
    fn from_field(field: &syn::Field) -> Result<Option<Self>> {
        let name: Ident =  field.ident
            .clone()
            .ok_or(Error::new(Span::call_site(), Problem::UnnamedField))?;

        let work = Work::try_from(field.attrs.as_slice())?;

        match work {
            Work { special: Some(Action::Skip), docs: _ } => Ok(None),
            Work { special: Some(Action::Rename(ident)), docs } => {
                Ok(Some(Field {
                    ty: field.ty.clone(),
                    name,
                    getter: ident,
                    docs
                }))
            },
            Work { special: None, docs } => {
                Ok(Some(Field {
                    ty: field.ty.clone(),
                    name: name.clone(),
                    getter: name,
                    docs,
                }))
            },
        }
    }
    
    fn from_fields_named(fields_named: &FieldsNamed) -> Result<Vec<Self>> {
        fields_named.named
            .iter()
            .try_fold(Vec::new(), |mut fields, field| {
                if let Some(field) = Field::from_field(field)? {
                    fields.push(field);
                }

                Ok(fields)
            })
    }

    fn emit(&self, struct_name: &Ident) -> TokenStream {
        let returns = &self.ty;
        let field_name = &self.name;
        let getter_name = &self.getter;
        
        let doc_comments: Vec<TokenStream> = if self.docs.is_empty() {
            let comment = format!(
                " Get field `{}` from instance of `{}`.",
                field_name,
                struct_name,
            );
            
            vec![quote!(#[doc=#comment])]
        } else {
            self.docs
                .iter()
                .map(|d| d.0.to_owned())
                .collect()
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
            },
            _ => {
                quote!(
                    #(#doc_comments)*
                    pub fn #getter_name(&self) -> &#returns {
                        &self.#field_name
                    }
                )
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
        let (impl_generics, struct_generics, where_clause) = self.original.generics
            .split_for_impl();        
        let struct_name = &self.name;
        let methods: Vec<TokenStream> = self.fields
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
        let named_fields = named_fields(struct_data)?;
        let fields = Field::from_fields_named(named_fields)?;

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
