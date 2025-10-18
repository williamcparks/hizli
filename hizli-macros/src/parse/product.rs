use hizli_core::FieldType;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Field};

pub fn init(field: &Field) -> TokenStream {
    match field.ident.as_ref() {
        Some(id) => quote! { #id: input.parse()? },
        None => quote! { input.parse()? },
    }
}

pub fn product(s: DataStruct) -> TokenStream {
    let field_type = FieldType::new(&s.fields);

    let inits = s.fields.iter().map(init);

    let init = field_type.wrap(quote! { #(#inits),* });

    quote! {
        ::core::result::Result::Ok(Self #init)
    }
}
