//! Dissolve internals
use std::{convert::TryFrom, iter::Extend};

use proc_macro2::{Delimiter, Group, Span, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
    AttrStyle, Attribute, DataStruct, DeriveInput, Error, Fields, FieldsNamed, FieldsUnnamed,
    Ident, Index, LitStr, Result, Type, TypeTuple,
};

use crate::{extract::named_struct, faultmsg::Problem};

pub enum IndexOrName {
    Index(Index),
    Name(Ident),
}

pub struct Field {
    ty: Type,
    name: IndexOrName,
}

impl Field {
    fn from_field(field: &syn::Field) -> Result<Self> {
        let name: Ident = field
            .ident
            .clone()
            .ok_or(Error::new(Span::call_site(), Problem::UnnamedField))?;

        Ok(Field {
            ty: field.ty.clone(),
            name: IndexOrName::Name(name),
        })
    }

    fn from_fields_named(fields_named: &FieldsNamed) -> Result<Vec<Self>> {
        fields_named.named.iter().map(Field::from_field).collect()
    }

    fn from_fields_unnamed(fields_unnamed: &FieldsUnnamed) -> Result<Vec<Self>> {
        fields_unnamed
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, field)| {
                Ok(Field {
                    ty: field.ty.clone(),
                    name: IndexOrName::Index(Index::from(i)),
                })
            })
            .collect()
    }

    fn from_struct(structure: &DataStruct) -> Result<Vec<Self>> {
        match structure.fields {
            Fields::Named(ref fields) => Self::from_fields_named(fields),
            Fields::Unnamed(ref fields) => Self::from_fields_unnamed(fields),
            Fields::Unit => Err(Error::new(Span::call_site(), Problem::UnitStruct)),
        }
    }
}

struct Rename {
    name: Ident,
}

impl Parse for Rename {
    fn parse(input: ParseStream) -> Result<Self> {
        syn::custom_keyword!(rename);

        if input.peek(rename) {
            let _ = input.parse::<rename>()?;
            let _ = input.parse::<syn::Token![=]>()?;
            let name = input.parse::<LitStr>()?;
            if !input.is_empty() {
                Err(Error::new(Span::call_site(), Problem::TokensFollowNewName))
            } else {
                let name = Ident::new(name.value().as_str(), Span::call_site());
                Ok(Rename { name })
            }
        } else {
            Err(Error::new(Span::call_site(), Problem::InvalidAttribute))
        }
    }
}

fn dissolve_rename_from(attributes: &[Attribute]) -> Result<Option<Ident>> {
    let mut current: Option<Ident> = None;

    for attr in attributes {
        if attr.style != AttrStyle::Outer {
            continue;
        }

        if attr.path().is_ident("dissolve") {
            let rename = attr.parse_args::<Rename>()?;
            current = Some(rename.name);
        }
    }

    Ok(current)
}

pub struct NamedStruct<'a> {
    original: &'a DeriveInput,
    name: Ident,
    fields: Vec<Field>,
    dissolve_rename: Option<Ident>,
}

impl<'a> NamedStruct<'a> {
    pub fn emit(&self) -> TokenStream {
        let (impl_generics, struct_generics, where_clause) =
            self.original.generics.split_for_impl();
        let struct_name = &self.name;

        let types: Punctuated<Type, syn::Token![,]> =
            self.fields.iter().fold(Punctuated::new(), |mut p, field| {
                p.push(field.ty.clone());
                p
            });

        let types_len = types.len();

        let return_type = if types_len > 1 {
            let tup_group = Group::new(Delimiter::Parenthesis, quote!(#types));
            let type_tuple = TypeTuple {
                paren_token: Paren {
                    span: tup_group.delim_span(),
                },
                elems: types,
            };

            quote!(#type_tuple)
        } else {
            if let Some(elem) = types.first() {
                quote!(#elem)
            } else {
                quote!(())
            }
        };

        let fields: TokenStream =
            self.fields
                .iter()
                .enumerate()
                .fold(TokenStream::new(), |mut ts, (count, field)| {
                    if count > 0 {
                        ts.extend(quote!(,))
                    }

                    let field_name = &field.name;
                    let field_expr = match field_name {
                        IndexOrName::Name(name) => {
                            quote!(
                                self.#name
                            )
                        }
                        IndexOrName::Index(i) => {
                            quote!(
                                self.#i
                            )
                        }
                    };

                    ts.extend(field_expr);

                    ts
                });

        let body = if types_len > 0 {
            quote! { ( #fields ) }
        } else {
            // Don't output `()` to avoid a compiler warning on an empty struct
            TokenStream::new()
        };

        let dissolve = Ident::new("dissolve", Span::call_site());
        let fn_name = self.dissolve_rename.as_ref().unwrap_or(&dissolve);

        let impl_comment = " Auto-generated by `derive_getters::Dissolve`.";
        let impl_doc_comment = quote!(#[doc=#impl_comment]);

        let fn_comment = format!(
            " Dissolve `{}` into a tuple consisting of its fields in order of declaration.",
            struct_name,
        );
        let fn_doc_comment = quote!(#[doc=#fn_comment]);

        quote!(
            #impl_doc_comment
            impl #impl_generics #struct_name #struct_generics
                #where_clause
            {
                #fn_doc_comment
                pub fn #fn_name(self) -> #return_type {
                    #body
                }
            }
        )
    }
}

impl<'a> TryFrom<&'a DeriveInput> for NamedStruct<'a> {
    type Error = Error;

    fn try_from(node: &'a DeriveInput) -> Result<Self> {
        let struct_data = named_struct(node)?;
        let fields = Field::from_struct(struct_data)?;
        let rename = dissolve_rename_from(node.attrs.as_slice())?;

        Ok(NamedStruct {
            original: node,
            name: node.ident.clone(),
            fields,
            dissolve_rename: rename,
        })
    }
}
