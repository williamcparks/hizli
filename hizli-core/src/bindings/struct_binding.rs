use syn::Fields;

use crate::{FieldBinding, FieldType};

/// Represents a struct or tuple struct’s field bindings and layout.
///
/// Wraps a set of [`FieldBinding`]s together with the corresponding
/// [`FieldType`], enabling consistent code generation across different
/// struct forms (unit, named, unnamed).
#[derive(Clone)]
pub struct StructBinding {
    field_bindings: Vec<FieldBinding>,
    field_type: FieldType,
}

impl StructBinding {
    /// Constructs a new [`StructBinding`] from a [`syn::Fields`] node.
    pub fn new(fields: &Fields) -> Self {
        Self {
            field_bindings: FieldBinding::from_fields(fields),
            field_type: FieldType::new(fields),
        }
    }

    /// Returns all [`FieldBinding`]s belonging to this struct.
    pub fn field_bindings(&self) -> &[FieldBinding] {
        &self.field_bindings
    }

    /// Returns the [`FieldType`] describing this struct’s layout.
    pub fn field_type(&self) -> FieldType {
        self.field_type
    }
}
