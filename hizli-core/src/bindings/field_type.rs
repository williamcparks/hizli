use proc_macro2::TokenStream;
use quote::quote;
use syn::Fields;

/// Describes the structural layout of a type’s fields.
///
/// Used by [`StructBinding`](`crate::StructBinding`) and [`VariantBinding`](`crate::VariantBinding`) to determine how to wrap
/// code fragments in parentheses, braces, or nothing when generating patterns
/// or construction expressions.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum FieldType {
    Unit,
    Named,
    Unnamed,
}

impl FieldType {
    /// Infers the [`FieldType`] from a [`syn::Fields`] AST node.
    pub fn new(fields: &Fields) -> Self {
        match fields {
            Fields::Unit => Self::Unit,
            Fields::Named(_) => Self::Named,
            Fields::Unnamed(_) => Self::Unnamed,
        }
    }

    /// Wraps a token stream in delimiters corresponding to the field type.
    ///
    /// - `Unit` leaves tokens unwrapped.  
    /// - `Named` wraps tokens in `{ ... }`.  
    /// - `Unnamed` wraps tokens in `( ... )`.
    pub fn wrap(&self, inner: TokenStream) -> TokenStream {
        match self {
            Self::Unit => inner,
            Self::Named => quote! { { #inner } },
            Self::Unnamed => quote! { ( #inner ) },
        }
    }
}
