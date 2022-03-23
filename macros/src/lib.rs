use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_quote;
use syn::spanned::Spanned;
use utils::MacroError;

mod command;
mod utils;

/// Procedural macro used to transform functions into commands
/// produces functions which can be passed to [`Framework::command`](ataraxy::Framework::command)
/// # Examples
/// ```rust, no_run
/// # extern crate ataraxy;
/// # use ataraxy::{ Framework, Context, command };
///
/// // Command Description
/// #[command]
/// async fn greet(ctx: Context, name: Option<String>) {
///     ctx.reply(msg!("Hello, {}", name.unwrap_or("Joe".to_string())))
/// }
/// // ... snip ...
/// # fn main() {
/// let framework = Framework::new()
///                     .command(greet);
/// # }
/// ```
#[proc_macro_attribute]
pub fn command(args: TokenStream, function: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as Vec<syn::NestedMeta>);
    let args = match <command::CommandArgs as darling::FromMeta>::from_list(&args) {
        Ok(x) => x,
        Err(e) => return e.write_errors().into(),
    };

    let function = syn::parse_macro_input!(function as syn::ItemFn);

    match command::command(args, function) {
        Ok(x) => x,
        Err(e) => match e {
            MacroError::SynError(e) => e.to_compile_error().into(),
            MacroError::DarlingError(e) => e.write_errors().into(),
        },
    }
}

/// For use with command addition for IDEs that do not support proc macros changing
/// function signatures (looking at you clion)
#[proc_macro_attribute]
pub fn command_ide_arg_support(_args: TokenStream, function: TokenStream) -> TokenStream {
    let mut fun = syn::parse_macro_input!(function as syn::ImplItem);
    if let syn::ImplItem::Method(mut function) = fun {
        function.sig.inputs = parse_quote! { mut self, cmd: fn() -> Command };
        function.sig.generics = syn::Generics::default();
        return function.into_token_stream().into();
    } else {
        return syn::Error::new(fun.span(), "Cannot wrap this :(")
            .into_compile_error()
            .into();
    }
}
