use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Lit;

/// Struct to parse a list of values from an attribute in darling: `#[multiple("item1", "item2")]`
#[derive(Debug, Clone, Default)]
pub struct Multiple<T>(pub Vec<T>);
impl<T: darling::FromMeta> darling::FromMeta for Multiple<T> {
    fn from_list(items: &[::syn::NestedMeta]) -> darling::Result<Self> {
        items
            .iter()
            .map(|item| T::from_nested_meta(item))
            .collect::<darling::Result<Vec<T>>>()
            .map(Self)
    }

    fn from_value(value: &Lit) -> darling::Result<Self> {
        Ok(Self(Vec::from([T::from_value(value)?])))
    }
}

/// Quotes an Option<T>
pub fn quote_vec<T: ToTokens>(opt: &[T]) -> TokenStream {
    quote! {
        vec! [#(#opt),*]
    }
}

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
