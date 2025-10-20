use hizli_core::StructEnumOnly;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

use crate::parse::{product::product, sum::sum};

pub fn handler(input: DeriveInput) -> Result<TokenStream> {
    let ident = input.ident;
    let (impl_gen, type_gen, where_cl) = input.generics.split_for_impl();

    let block = match StructEnumOnly::try_new(input.data, "Parse")? {
        StructEnumOnly::Struct(s) => product(s),
        StructEnumOnly::Enum(e) => sum(e, &ident.to_string())?,
    };

    Ok(quote! {
        #[automatically_derived]
        impl #impl_gen ::syn::parse::Parse for #ident #type_gen #where_cl {
            fn parse(input: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
                #block
            }
        }
    })
}
