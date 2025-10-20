//! # Bindings Library
//!
//! This library provides a small, composable abstraction layer over the
//! [`syn`](https://docs.rs/syn) AST for working with fields, structs, and enum variants
//! in procedural macros.
//!
//! It defines a hierarchy of **binding types** that model Rust data declarations
//! as reusable, introspectable units that can easily be converted into
//! token streams for code generation or pattern matching.
//!
//! ## Core Concepts
//!
//! The system centers around three main abstractions:
//!
//! - [`FieldBinding`] — wraps a single field (named or unnamed) and provides
//!   both its [`Ident`](https://docs.rs/syn/latest/syn/struct.Ident.html) and [`Member`](https://docs.rs/syn/latest/syn/enum.Member.html) for easy access in generated code.
//! - [`StructBinding`] — aggregates multiple `FieldBinding`s from a struct or tuple struct
//!   and determines the enclosing [`FieldType`] (named, unnamed, or unit).
//! - [`VariantBinding`] — extends `StructBinding` to enum variants,
//!   providing both the variant identifier and its field bindings.
//! - [`NsAttr`] & [`AttrLevel`] provide helpers for creating "Namespaced" attributes.
//!
//! Together these types make it easy to:
//!
//! - Generate destructuring patterns and match arms from parsed `syn::Data`.
//! - Write derive macros that support both structs and enums uniformly.
//! - Avoid repetitive pattern code for different struct and variant shapes.
//!
//! ## Field Layout Handling
//!
//! The [`FieldType`] enum classifies field arrangements:
//!
//! - `Unit` → no fields  
//! - `Named` → `{ a, b }`-style fields  
//! - `Unnamed` → `(a, b)`-style tuple fields  
//!
//! The [`FieldType::wrap`] helper can automatically surround token streams
//! in the correct delimiters, simplifying code generation for pattern bindings
//! and constructor calls.
//!
//! ## Example Usage, Simple "MyClone" Macro
//!
//! ```rust
//! use syn::{parse_quote, DeriveInput};
//! use bindings::{VariantBinding, StructBinding};
//!
//! // Example: derive macro processing a struct
//! let input: DeriveInput = parse_quote! {
//!     struct Example { a: u32, b: String }
//! };
//!
//! let StructOnly(s) = StructOnly::try_new(input.data, "MyClone")?;
//!
//! let inits = s.fields.members().map(|member| {
//!     match member {
//!         Member::Named(id) => quote! { #id: self.#id.my_clone() },
//!         Member::Unnamed(idx) => quote! { self.#idx.my_clone() },
//!     }
//! });
//!
//! let init = FieldType::new(&s.fields).wrap(quote! {
//!     #(#inits),*
//! });
//!
//! quote! {
//!     Self #init
//! }
//!
//! ```
//!
//! ## Module Summary
//!
//! | Module | Description |
//! |---------|-------------|
//! | `field_binding` | Defines [`FieldBinding`], representing individual struct or variant fields. |
//! | `field_type` | Defines [`FieldType`], describing named, unnamed, or unit layouts. |
//! | `struct_binding` | Groups fields into a [`StructBinding`] with consistent layout info. |
//! | `variant_binding` | Wraps enum variants into [`VariantBinding`] for uniform field access. |
//! | `variant_pattern` | Provides `VariantBinding::variant_pattern()` for generating binding patterns. |
//!
//! ## Intended Use
//!
//! Designed for derive macro and codegen authors who need minimal, zero-dependency
//! helpers to represent and emit token-level representations of `syn` AST nodes
//! without repetitive boilerplate. The API is purely structural and does not depend
//! on specific derive semantics, making it a general-purpose tool for code generation
//! pipelines.
//!
//! # Procedural Macros: `#[derive(Parse)]` and `#[derive(Spanable)]`
//!
//! This crate provides two custom derive macros that automatically implement
//! common parsing and span-extraction traits for structs and enums using
//! the [`syn`](https://docs.rs/syn) and [`quote`](https://docs.rs/quote) crates.
//!
//! These derive macros are designed to work seamlessly with the helper types
//! defined in **`hizli_core`**, such as [`StructEnumOnly`], [`FieldType`],
//! and [`VariantBinding`].
//!
//! ---
//!
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
//! Implements the `span(&self)` method.
//!
//! - For **structs**, it returns the span of the first field if one exists,
//!   or the call-site span if the struct has no fields.  
//! - For **enums**, it generates a `match` expression returning the span of
//!   the first field of each variant, or the call-site span for unit variants.
//!
//! #### Example
//! ```rust
//! use hizli_macros::Spanable;
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
//! let span = leaf.span();
//! ```

pub use hizli_core::*;
pub use hizli_macros::{Parse, Spanable};
