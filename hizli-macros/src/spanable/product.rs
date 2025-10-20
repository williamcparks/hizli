use proc_macro2::TokenStream;
use quote::quote;
use syn::DataStruct;

pub fn product(s: DataStruct) -> TokenStream {
    match s.fields.members().next() {
        Some(member) => quote! { self.#member.span() },
        None => quote! { ::proc_macro2::Span::call_site() },
    }
}
