//! Binding utilities for struct and enum pattern generation in procedural macros.
//!
//! This module provides a set of lightweight abstractions for representing and
//! manipulating Rust data structure fields and variants (`struct`, `tuple`, and `enum`)
//! as reusable “bindings.” These types allow derive and macro expansion code to treat
//! fields and variants uniformly when generating tokens, building match patterns, or
//! performing code analysis.
//!
//! ## Overview
//!
//! | Type | Purpose |
//! |------|----------|
//! | [`FieldBinding`] | Represents a single field binding (named or positional). |
//! | [`FieldType`] | Describes the field layout (`Unit`, `Named`, or `Unnamed`). |
//! | [`StructBinding`] | Aggregates field bindings for a struct or tuple struct. |
//! | [`VariantBinding`] | Wraps an enum variant with its fields and layout info. |
//!
//! These are composed to support flexible generation of token streams for patterns,
//! destructuring, and initialization in procedural macros.

mod field_binding;
mod field_type;
mod struct_binding;
mod variant_binding;
mod variant_pattern;

pub use field_binding::FieldBinding;
pub use field_type::FieldType;
pub use struct_binding::StructBinding;
pub use variant_binding::VariantBinding;
