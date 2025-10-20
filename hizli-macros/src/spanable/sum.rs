use hizli_core::VariantBinding;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Variant};

fn arm(variant: &Variant) -> TokenStream {
    let binding = VariantBinding::new(variant);
    let pat = binding.variant_pattern();

    let expr = match binding.field_bindings().iter().next() {
        Some(some) => {
            let ident = some.ident();
            quote! { #ident.span() }
        }
        None => quote! { ::proc_macro2::Span::call_site() },
    };

    quote! {
        Self::#pat => #expr
    }
}

pub fn sum(e: DataEnum) -> TokenStream {
    if e.variants.is_empty() {
        return quote! { match *self {} };
    }

    let arms = e.variants.iter().map(arm);

    quote! {
        match self {
            #(#arms),*
        }
    }
}
