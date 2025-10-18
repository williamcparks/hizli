use syn::{Data, DataEnum, DataStruct, Error, Result};

/// Represents a `syn::Data` node restricted to only `struct` or `enum` variants.
///
/// This is useful in procedural macros that should reject `union` definitions
/// when deriving traits or implementing code generation logic.
pub enum StructEnumOnly {
    Struct(DataStruct),
    Enum(DataEnum),
}

impl StructEnumOnly {
    /// Attempts to construct a [`StructEnumOnly`] from a [`syn::Data`] value.
    ///
    /// # Parameters
    /// - `data`: The `syn::Data` extracted from a parsed item (usually from a [`syn::DeriveInput`]).
    /// - `derive_name`: The name of the current derive macro, used in error messages.
    ///
    /// # Usage
    ///
    /// ```no_run
    /// let input: DeriveInput = ...;
    ///
    /// match StructEnumOnly::try_new(input.data, "MyMacro")? {
    ///     StructEnumOnly::Struct(s) => todo!(),
    ///     StructEnumOnly::Enum(s) => todo!(),
    /// }
    /// ```
    pub fn try_new(data: Data, derive_name: &str) -> Result<Self> {
        match data {
            Data::Struct(s) => Ok(Self::Struct(s)),
            Data::Enum(e) => Ok(Self::Enum(e)),
            Data::Union(u) => Err(Error::new(
                u.union_token.span,
                format!("Cannot #[derive({derive_name})] On Union"),
            )),
        }
    }
}

/// Wrapper around [`syn::DataStruct`] that rejects any non-struct input.
///
/// Intended for derive macros or helpers that operate **only** on structs.
pub struct StructOnly(pub DataStruct);

impl StructOnly {
    /// Attempts to construct a [`StructOnly`] from a [`syn::Data`] value.
    ///
    /// # Parameters
    /// - `data`: The `syn::Data` node extracted from a parsed item.
    /// - `derive_name`: The name of the current derive macro, used in error diagnostics.
    ///
    ///  # Usage
    /// ```no_run
    /// let input: DeriveInput = ...;
    ///
    /// let StructOnly(data) = StructOnly::try_new(input.data, "MyMacro")?;
    /// ```
    pub fn try_new(data: Data, derive_name: &str) -> Result<Self> {
        match data {
            Data::Struct(s) => Ok(Self(s)),
            Data::Enum(e) => Err(Error::new(
                e.enum_token.span,
                format!("Cannot #[derive({derive_name})] On Enum"),
            )),
            Data::Union(u) => Err(Error::new(
                u.union_token.span,
                format!("Cannot #[derive({derive_name})] On Union"),
            )),
        }
    }
}

/// Wrapper around [`syn::DataEnum`] that rejects any non-enum input.
///
/// Intended for derive macros or helpers that operate **only** on enums.
pub struct EnumOnly(pub DataEnum);

impl EnumOnly {
    /// Attempts to construct a [`EnumOnly`] from a [`syn::Data`] value.
    ///
    /// # Parameters
    /// - `data`: The `syn::Data` node extracted from a parsed item.
    /// - `derive_name`: The name of the current derive macro, used in error diagnostics.
    ///
    ///  # Usage
    /// ```no_run
    /// let input: DeriveInput = ...;
    ///
    /// let EnumOnly(data) = EnumOnly::try_new(input.data, "MyMacro")?;
    /// ```
    pub fn try_new(data: Data, derive_name: &str) -> Result<Self> {
        match data {
            Data::Enum(e) => Ok(Self(e)),
            Data::Struct(s) => Err(Error::new(
                s.struct_token.span,
                format!("Cannot #[derive({derive_name})] On Struct"),
            )),
            Data::Union(u) => Err(Error::new(
                u.union_token.span,
                format!("Cannot #[derive({derive_name})] On Union"),
            )),
        }
    }
}
