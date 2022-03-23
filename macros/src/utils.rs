use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

/// Quotes an Option<T>
pub fn quote_option<T: ToTokens>(opt: &Option<T>) -> TokenStream {
    match opt {
        Some(o) => quote! {Some( #o )},
        None => quote! {None},
    }
}

/// Enum to wrap syn and darling errors
pub enum MacroError {
    DarlingError(darling::Error),
    SynError(syn::Error),
}
