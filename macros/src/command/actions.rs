use proc_macro2::TokenStream;
use quote::quote;

fn create_slash_command_action(num_args: u8) -> TokenStream {
    let args: Vec<&str> = (1..num_args)
        .into_iter()
        .map(|n| &*format!("arg{}", n))
        .collect();
    quote! {
        |ctx, args| {
            let mut args = args.clone();
            #(let #args = args.next());*

            inner(ctx, #(#args.into_arg()),*);
        }
    }
}
