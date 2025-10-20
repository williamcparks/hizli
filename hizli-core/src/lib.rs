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
//!   both its [`syn::Ident`] and [`syn::Member`] for easy access in generated code.
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
//! ## Example Usage
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
//! if let syn::Data::Struct(data) = input.data {
//!     let binding = StructBinding::new(&data.fields);
//!     for field in binding.field_bindings() {
//!         println!("field name = {}", field.ident());
//!     }
//! }
//!
//! // Example: generating a pattern for an enum variant
//! let variant = parse_quote! { Foo(u8, u8) };
//! let vb = VariantBinding::new(&variant);
//! println!("{}", vb.variant_pattern());
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

mod bindings;
mod data;
mod ns_attr;
mod rules;

pub use bindings::{FieldBinding, FieldType, StructBinding, VariantBinding};
pub use data::{EnumOnly, StructEnumOnly, StructOnly};
pub use ns_attr::{AttrLevel, NsAttr};
