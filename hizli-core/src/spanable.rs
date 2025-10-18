use proc_macro2::Span;
use syn::spanned::Spanned;

/// Provides a universal way to extract a [`proc_macro2::Span`] from types
///
/// ## Why
/// The [`Spanned`] trait in `syn` is currently **sealed**, meaning it cannot be
/// implemented for custom types outside the crate. This restriction limits
/// downstream crates that need to work generically with types that *have* a span
/// but are not directly recognized by `syn`.
pub trait Spanable {
    fn spanable(&self) -> Span;
}

impl<T: Spanned> Spanable for T {
    fn spanable(&self) -> Span {
        Spanned::span(self)
    }
}
