//! This library provides two derive macros. One, `Getters` for autogenerating getters and
//! `Dissolve` for consuming a struct returning a tuple of all fields. They can only be
//! used on named structs.
//!
//! # Derives
//!
//! Only named structs can derive `Getters` or `Dissolve`.
//!
//! # `Getter` methods generated
//!
//! The getter methods generated shall bear the same name as the struct fields and be
//! publicly visible. The methods return an immutable reference to the struct field of the
//! same name. If there is already a method defined with that name there'll be a collision.
//! In these cases one of two attributes can be set to either `skip` or `rename` the getter.
//!
//!
//! # `Getters` Usage
//!
//! In lib.rs or main.rs;
//!
//!```edition2021
//! use derive_getters::Getters;
//!
//! #[derive(Getters)]
//! struct User {
//!     name: String,
//! }
//!
//! let user = User { name: "John Doe".to_string() };
//! assert!(user.name() == "John Doe");
//! ```
//!
//! Here, a method called `num()` has been created for the `Number` struct which gives a
//! reference to the `num` field.
//!
//! This macro can also derive on structs that have simple generic types. For example;
//!
//! ```edition2021
//! # use derive_getters::Getters;
//! #[derive(Getters)]
//! struct Generic<T, U> {
//!     gen_t: T,
//!     gen_u: U,
//! }
//! #
//! # fn main() { }
//! ```
//!
//! The macro can also handle generic types with trait bounds. For example;
//! ```edition2021
//! # use derive_getters::Getters;
//! #[derive(Getters)]
//! struct Generic<T: Clone, U: Copy> {
//!     gen_t: T,
//!     gen_u: U,
//! }
//! #
//! # fn main() { }
//! ```
//! The trait bounds can also be declared in a `where` clause.
//!
//! Additionaly, simple lifetimes are OK too;
//! ```edition2021
//! # use derive_getters::Getters;
//! #[derive(Getters)]
//! struct Annotated<'a, 'b, T> {
//!     stuff: &'a T,
//!     comp: &'b str,
//!     num: u64,
//! }
//! #
//! # fn main() { }
//! ```
//!
//! # `Getter` Attributes
//! Getters can be further configured to either skip or rename a getter.
//!
//! * #[getter(skip)]
//! Will skip generating a getter for the field being decorated.
//!
//! * #[getter(rename = "name")]
//! Changes the name of the getter (default is the field name) to "name".
//!
//!```edition2021
//! # use derive_getters::Getters;
//! #[derive(Getters)]
//! struct Attributed {
//!     keep_me: u64,
//!
//!     #[getter(skip)]
//!     skip_me: u64,
//!
//!     #[getter(rename = "number")]
//!     rename_me: u64,
//! }
//! #
//! # fn main() { }
//! ```
//!
//! # `Dissolve` method generated
//!
//! Deriving `Dissolve` on a named or unit struct will generate a method `dissolve(self)`
//! which shall return a tuple of all struct fields in the order they were defined. Calling
//! this method consumes the struct. The name of this method can be changed with an
//! attribute.
//!
//! # `Dissolve` usage
//!
//! ```edition2021
//! # use derive_getters::Dissolve;
//! #[derive(Dissolve)]
//! struct Stuff {
//!     name: String,
//!     price: f64,
//!     count: usize,
//! }
//!
//! fn main() {
//!     let stuff = Stuff {
//!         name: "Hogie".to_owned(),
//!         price: 123.4f64,
//!         count: 100,
//!     };
//!
//!     let (n, p, c) = stuff.dissolve();
//!     assert!(n == "Hogie");
//!     assert!(p == 123.4f64);
//!     assert!(c == 100);
//! }
//! ```
//!
//! # `Dissolve` can be derived on tuple structs.
//!
//! ```edition2021
//! # use derive_getters::Dissolve;
//! #[derive(Dissolve)]
//! struct Stuff(String, f64, usize);
//!
//! fn main() {
//!     let stuff = Stuff("Hogie".to_owned(), 123.4f64, 100);
//!
//!     let (n, p, c) = stuff.dissolve();
//!     assert!(n == "Hogie");
//!     assert!(p == 123.4f64);
//!     assert!(c == 100);
//! }
//! ```
//!
//! # `Dissolve` Attributes
//! You can rename the `dissolve` function by using a struct attribute.
//!
//! * #[dissolve(rename = "name")]
//!
//! ```edition2021
//! # use derive_getters::Dissolve;
//! #[derive(Dissolve)]
//! #[dissolve(rename = "shatter")]
//! struct Numbers {
//!     a: u64,
//!     b: i64,
//!     c: f64,
//! }
//! #
//! # fn main() { }
//! ```
//!
//! # Comment Replication/Generation
//!
//! Comments are produced for the auto-generated getters or dissolver. A comment is also
//! generated for the impl block.
//!
//! ## Replication of comments
//!
//! Any field comments are replicated for the getter. If the field on the target struct
//! has a comment; the getter for it shall have the exact same comment.
//!
//! ```edition2021
//! # use derive_getters::Getters;
//! #[derive(Getters)]
//! struct Number {
//!     /// My special number.
//!     num: u64,
//! }
//! #
//! # fn main() { }
//!```
//!
//! ## Generation of comments
//!
//! If no comment is present for the field, one shall be generated like so;
//! " Get field `{}` from instance of `{}`."
//!
//! A comment for the dissolve function shall be similarily generated;
//! "Dissolve `{}` into a tuple consisting of its fields in order of declaration."
//!
//! The impl block for the getters or dissolve function also has a comment generated;
//! "Auto-generated by `derive_getters::Getters`." and or
//! "Auto-generated by `derive_getters::Dissolve`".
//!
//! # Panics
//!
//! If `Getters` is derived on unit, unnamed structs, enums or unions.
//! If `Dissolve` is dervied on unnamed structs, enums or unions.
//!
//! # Cannot Do
//! Const generics aren't handled by this macro nor are they tested.
use std::convert::TryFrom;

extern crate proc_macro;
use syn::{parse_macro_input, DeriveInput};

mod dissolve;
mod extract;
mod faultmsg;
mod getters;

/// Generate getter methods for all named struct fields in a seperate struct `impl` block.
/// Getter methods share the name of the field they're 'getting'. Methods return an
/// immutable reference to the field.
#[proc_macro_derive(Getters, attributes(getter))]
pub fn getters(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    getters::NamedStruct::try_from(&ast)
        .map(|ns| ns.emit())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Produce a `dissolve` method that consumes the named struct returning a tuple of all the
/// the struct fields.
#[proc_macro_derive(Dissolve, attributes(dissolve))]
pub fn dissolve(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    dissolve::NamedStruct::try_from(&ast)
        .map(|ns| ns.emit())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
