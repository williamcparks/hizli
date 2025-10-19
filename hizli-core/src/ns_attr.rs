use proc_macro2::Span;
use syn::{Attribute, Error, Result, parse::Parse, spanned::Spanned};

/// Indicates the syntactic level an attribute applies to.
///
/// Used for context-aware validation in procedural macros.  
/// - `Type` for attributes applied at the struct/enum level.  
/// - `Variant` for attributes applied to enum variants.  
/// - `Field` for attributes applied to struct fields.
#[derive(Clone, Copy, Debug)]
pub enum AttrLevel {
    Type,
    Variant,
    Field,
}

/// Trait for attributes with a fixed namespace identifier.
///
/// Implementors define a `const NS` string identifying the attribute
/// namespace (for example, `"myattr"` for `#[myattr(...)]`), and
/// implement [`Parse`] to handle their specific argument structure.
///
/// This trait provides helper methods for extracting or rejecting
/// attributes matching that namespace from a list of [`syn::Attribute`].
pub trait NsAttr: Parse {
    /// The namespace identifier (attribute name).
    ///
    /// Example:
    /// ```ignore
    /// const NS: &str = "myattr";
    /// ```
    const NS: &str;

    /// Attempts to parse the namespaced attribute from a list of attributes.
    ///
    /// Returns:
    /// - `Ok(Some(Self))` if the attribute was found and parsed.
    /// - `Ok(None)` if no matching attribute was present.
    /// - `Err` if multiple instances of the same attribute are found or parsing fails.
    ///
    /// Example:
    /// ```ignore
    /// let parsed = MyAttr::from_attrs_opt(&input.attrs)?;
    /// if let Some(attr) = parsed {
    ///     // use attr
    /// }
    /// ```
    fn from_attrs_opt(attrs: &[Attribute]) -> Result<Option<Self>> {
        let mut res = None;
        for attr in attrs {
            if !attr.path().is_ident(Self::NS) {
                continue;
            }
            if res.is_some() {
                return Err(Error::new(
                    attr.span(),
                    format!("Attribute #[{}] Is Already Configured", Self::NS),
                ));
            }
            res = Some(attr.parse_args()?);
        }
        Ok(res)
    }

    /// Parses a required namespaced attribute from a list of attributes.
    ///
    /// This method behaves like [`NsAttr::from_attrs_opt`], but instead of
    /// returning `Ok(None)` when the attribute is missing, it emits an error.
    ///
    /// # Parameters
    /// - `attrs`: The list of [`syn::Attribute`] values to search through.
    /// - `span`: The [`proc_macro2::Span`] used for error reporting when the
    ///   required attribute is not found.
    fn from_attrs(attrs: &[Attribute], span: Span) -> Result<Self> {
        match Self::from_attrs_opt(attrs)? {
            Some(some) => Ok(some),
            None => Err(Error::new(
                span,
                format!("Attribute #[{}] Is Required", Self::NS),
            )),
        }
    }

    /// Ensures that the given attributes contain no occurrence of this namespace.
    ///
    /// Used to enforce that an attribute is *not allowed* at a given syntactic level.
    ///
    /// Returns an error if the attribute is found.
    ///
    /// Example:
    /// ```ignore
    /// MyAttr::no_attrs(&variant.attrs, AttrLevel::Variant)?;
    /// ```
    fn no_attrs(attrs: &[Attribute], level: AttrLevel) -> Result<()> {
        match attrs.iter().find(|a| a.path().is_ident(Self::NS)) {
            None => Ok(()),
            Some(attr) => Err(Error::new(
                attr.span(),
                format!(
                    "Attribute #[{}] Is Not Allowed At The {level:?} Level",
                    Self::NS
                ),
            )),
        }
    }
}
