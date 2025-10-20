use hizli_core::StructEnumOnly;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

use crate::spanable::{product::product, sum::sum};

pub fn handler(input: DeriveInput) -> Result<TokenStream> {
    let ident = input.ident;

    let (impl_gen, type_gen, where_cl) = input.generics.split_for_impl();

    let block = match StructEnumOnly::try_new(input.data, "Spanable")? {
        StructEnumOnly::Enum(e) => sum(e),
        StructEnumOnly::Struct(s) => product(s),
    };

    Ok(quote! {
        #[automatically_derived]
        impl #impl_gen #ident #type_gen #where_cl {
            fn spanable(&self) -> ::proc_macro2::Span {
                #block
            }
        }
    })
}
