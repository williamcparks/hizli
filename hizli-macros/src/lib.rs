//! ## Macros
//!
//! ### `#[derive(Parse)]`
//!
//! Implements the [`syn::parse::Parse`](https://docs.rs/syn/latest/syn/parse/trait.Parse.html) trait for your type.  
//!
//! - **Structs** are parsed field-by-field using `input.parse()?` for each field.  
//! - **Enums** are parsed by peeking at the next token and choosing the matching
//!   variant, returning a helpful error message if no variant matches.  
//!
//! #### Supported forms
//! - Named structs (`struct Foo { a: A, b: B }`)  
//! - Tuple structs (`struct Foo(A, B)`)  
//! - Enums with variants containing fields or tuples
//!
//! #### Example
//! ```rust
//! use hizli_macros::Parse;
//! use syn::{Token, Ident};
//!
//! #[derive(Parse)]
//! struct Pair {
//!     left: Ident,
//!     _comma: Token![,],
//!     right: Ident,
//! }
//!
//! // input: syn::parse::ParseStream
//! let pair: Pair = input.parse()?;
//! ```
//!
//! ---
//!
//! ### `#[derive(Spanable)]`
//!
//! Implements the `hizli::Spanable` trait, which provides a `spanable(&self) -> proc_macro2::Span` method.
//!
//! - For **structs**, it returns the span of the first field if one exists,
//!   or the call-site span if the struct has no fields.  
//! - For **enums**, it generates a `match` expression returning the span of
//!   the first field of each variant, or the call-site span for unit variants.
//!
//! #### Example
//! ```rust
//! use hizli_macros::Spanable;
//! use hizli::Spanable;
//! use proc_macro2::Span;
//! use syn::LitStr;
//!
//! #[derive(Spanable)]
//! enum Example {
//!     LitStr(LitStr),
//!     Other,
//! }
//!
//! let leaf = Node::Other;
//! let span = leaf.spanable();
//! ```

use hizli_core::out;
use proc_macro::TokenStream;

mod parse;
mod spanable;

/// Derive macro that automatically implements [`syn::parse::Parse`] for structs and enums.
///
/// This derive generates a `Parse` implementation suitable for use with the
/// [`syn::parse`](https://docs.rs/syn/latest/syn/parse/index.html) framework.
/// It supports both *product types* (structs) and *sum types* (enums).
#[proc_macro_derive(Parse)]
pub fn parse(input: TokenStream) -> TokenStream {
    out!(parse::handler::handler, input)
}

/// Derive macro that implements the `hizli::Spanable` trait for structs and enums.
#[proc_macro_derive(Spanable)]
pub fn spanable(input: TokenStream) -> TokenStream {
    out!(spanable::handler::handler, input)
}
