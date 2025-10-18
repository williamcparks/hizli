use proc_macro2::TokenStream;
use quote::quote;

use crate::VariantBinding;

impl VariantBinding {
    /// Generates a token pattern representing this variant’s binding form.
    ///
    /// For example, given an enum variant `Foo(a, b)`, this produces:
    ///
    /// ```text
    /// Foo(binding_0, binding_1)
    /// ```
    ///
    /// or for named fields:
    ///
    /// ```text
    /// Foo { field_a, field_b }
    /// ```
    ///
    /// depending on the variant’s [`FieldType`](`crate::FieldType`).
    pub fn variant_pattern(&self) -> TokenStream {
        let variant_id = self.ident();
        let bindings = self.field_bindings().iter().map(|fb| fb.ident());
        let pattern = self.field_type().wrap(quote! {
            #(#bindings),*
        });

        quote! {
            #variant_id #pattern
        }
    }
}
