use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn create_slash_command_action(ctx: bool, num_args: u8) -> TokenStream {
    let args: Vec<Ident> = (0..num_args)
        .into_iter()
        .map(|n| format_ident!("arg{}", n))
        .collect();
    let context = if ctx { quote!(ctx,) } else { quote!() };
    quote! {
        |ctx, args| {
            let mut args = args.clone().iter();
            // TODO: Better error msg
            #(let #args = args.next().expect("Ran out of provided arguments").clone();)*

            Box::pin(async move {
                inner(#context #(#args.as_arg()),*).await
            })
        }
    }
}
