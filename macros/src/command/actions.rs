use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn create_slash_command_action(ctx: bool, num_args: u8) -> TokenStream {
    let args: Vec<TokenStream> = (0..num_args)
        .into_iter()
        .map(|_| {
            quote! {
                args.arg(&ctx).await
            }
        })
        .collect();
    let context = if ctx { quote!(ctx.clone(),) } else { quote!() };
    quote! {
        |ctx, args| {
            let mut args = args.clone();

            Box::pin(async move {
                inner(#context #(#args),*).await
            })
        }
    }
}
