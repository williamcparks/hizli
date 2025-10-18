use syn::{Ident, Variant};

use crate::{FieldBinding, FieldType, StructBinding};

/// Represents a bound enum variant, including its name and fields.
///
/// Combines a variant identifier and its [`StructBinding`] so that enum
/// variants can be treated like structs in downstream code generation.
#[derive(Clone)]
pub struct VariantBinding {
    ident: Ident,
    struct_binding: StructBinding,
}

impl VariantBinding {
    /// Creates a new [`VariantBinding`] from a parsed [`syn::Variant`].
    pub fn new(variant: &Variant) -> Self {
        Self {
            ident: variant.ident.clone(),
            struct_binding: StructBinding::new(&variant.fields),
        }
    }

    /// Returns the identifier of the variant.
    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    /// Returns a reference to the field bindings of this variant.
    pub fn field_bindings(&self) -> &[FieldBinding] {
        self.struct_binding.field_bindings()
    }

    /// Returns the [`FieldType`] describing this variant’s field layout.
    pub fn field_type(&self) -> FieldType {
        self.struct_binding.field_type()
    }
}
