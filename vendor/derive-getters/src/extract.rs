//! Common functions

use proc_macro2::Span;
use syn::{FieldsNamed, DataStruct, DeriveInput, Data, Fields, Error, Result};

use crate::faultmsg::{StructIs, Problem};

pub fn named_fields(structure: &DataStruct) -> Result<&FieldsNamed> {
    match structure.fields {
        Fields::Named(ref fields) => Ok(fields),
        Fields::Unnamed(_) | Fields::Unit => Err(
            Error::new(Span::call_site(), Problem::UnnamedField)
        ),
    }
}

pub fn named_struct(node: &DeriveInput) -> Result<&DataStruct> {
    match node.data {
        Data::Struct(ref structure) => Ok(structure),
        Data::Enum(_) => Err(
            Error::new_spanned(node, Problem::NotNamedStruct(StructIs::Enum))
        ),
        Data::Union(_) => Err(
            Error::new_spanned(node, Problem::NotNamedStruct(StructIs::Union))
        ),
    }
}
