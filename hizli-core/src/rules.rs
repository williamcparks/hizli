/// A convenience macro for wrapping procedural macro entry points with uniform
/// error handling and output conversion.
///
/// # Syntax
///
/// ```no_run
/// #[proc_macro] // or #[proc_macro_derive(...)]
/// pub fn my_macro(input: ...) -> ... {
///     out!(path::to::my_handler, input)
/// }
///
/// use proc_macro2::TokenStream;
/// use quote::quote;
/// use syn::Result;
///
/// fn my_handler(input: SomeTypeThatImplsParse) -> Result<TokenStream> {
///     Ok(quote! {
///         // output
///     })
/// }
/// ```
#[macro_export]
macro_rules! out {
    ($handler: path, $input: tt) => {
        ::proc_macro::TokenStream::from(match $handler(::syn::parse_macro_input!($input)) {
            ::core::result::Result::Err(err) => ::syn::Error::into_compile_error(err),
            ::core::result::Result::Ok(ok) => ok,
        })
    };
}
