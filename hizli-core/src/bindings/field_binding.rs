use syn::{Field, Fields, Ident, Index, Member, spanned::Spanned};

/// Represents a single field binding within a struct, tuple struct, or enum variant.
///
/// Provides both the [`Ident`] (binding name used in generated code) and
/// [`Member`] (the AST member for accessing the field by name or index).
#[derive(Clone)]
pub struct FieldBinding {
    ident: Ident,
    member: Member,
}

impl FieldBinding {
    /// Creates a new [`FieldBinding`] from a field and its index position.
    ///
    /// - Named fields use their existing identifier.
    /// - Unnamed (tuple) fields are assigned synthetic identifiers
    ///   in the form of `binding_{index}`.
    pub fn new((idx, field): (usize, &Field)) -> Self {
        match field.ident.clone() {
            Some(ident) => {
                let member = Member::Named(ident.clone());
                Self { ident, member }
            }
            None => {
                let member = Member::Unnamed(Index {
                    index: idx.try_into().unwrap_or_default(),
                    span: field.span(),
                });
                let ident = format!("binding_{idx}");
                let ident = Ident::new(&ident, field.span());
                Self { ident, member }
            }
        }
    }

    /// Returns the identifier used for this binding in generated code.
    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    /// Returns the underlying [`Member`] corresponding to this field.
    pub fn member(&self) -> &Member {
        &self.member
    }

    /// Creates a vector of [`FieldBinding`]s for all fields in a [`syn::Fields`] node.
    pub fn from_fields(fields: &Fields) -> Vec<Self> {
        fields.iter().enumerate().map(Self::new).collect()
    }
}
