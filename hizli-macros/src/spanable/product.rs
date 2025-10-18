use proc_macro2::TokenStream;
use quote::quote;
use syn::DataStruct;

pub fn product(s: DataStruct) -> TokenStream {
    match s.fields.members().next() {
        Some(member) => quote! { ::hizli::Spanable::spanable(self.#member) },
        None => quote! { ::proc_macro2::Span::call_site() },
    }
}
