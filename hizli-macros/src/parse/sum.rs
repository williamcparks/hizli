use hizli_core::FieldType;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Error, Result, Variant};

use crate::parse::{product::init, sum_expected_one_of::sum_expected_one_of};

fn branch(variant: &Variant) -> Result<TokenStream> {
    let ident = &variant.ident;
    let first = match variant.fields.iter().next() {
        Some(some) => some,
        None => {
            return Err(Error::new(
                ident.span(),
                "#[derive(Parse)] Requires At Least One Field",
            ));
        }
    };
    let ty = &first.ty;

    let inits = variant.fields.iter().map(init);

    let init = FieldType::new(&variant.fields).wrap(quote! { #(#inits),* });

    Ok(quote! {
        if input.peek(#ty) {
            return ::core::result::Result::Ok(Self::#ident #init);
        }
    })
}

pub fn sum(e: DataEnum, id: &str) -> Result<TokenStream> {
    if e.variants.is_empty() {
        return Err(Error::new(
            e.enum_token.span,
            "Cannot #[derive(Parse)] On An Empty Enum. It's Not Constructable At Runtime",
        ));
    }
    let msg = sum_expected_one_of(&e, id);

    let branches = e.variants.iter().map(branch).collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        #(#branches)*

        ::core::result::Result::Err(::syn::Error::new(input.span(), #msg))
    })
}
