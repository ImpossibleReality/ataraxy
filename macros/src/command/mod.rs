mod actions;
mod params;

use crate::utils::MacroError;
use crate::utils::MacroError::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::Lit::Str;
use syn::{FnArg, ItemFn, Meta};

#[derive(Default, Debug, darling::FromMeta)]
#[darling(default)]
pub struct CommandArgs {
    name: Option<String>,
    description: Option<String>,
}

fn extract_doc_comments(function: &ItemFn) -> Option<String> {
    let mut doc_lines = String::new();
    for attr in &function.attrs {
        if attr.path == quote::format_ident!("doc").into() {
            if let Ok(Meta::NameValue(nv)) = attr.parse_meta() {
                if let Str(literal) = nv.lit {
                    let literal = literal.value();
                    let literal = literal.strip_prefix(' ').unwrap_or(&literal);

                    doc_lines += literal;
                    doc_lines += "\n";
                }
            }
        }
    }
    if doc_lines.is_empty() {
        None
    } else {
        Some(doc_lines)
    }
}

pub fn command(args: CommandArgs, function: ItemFn) -> Result<TokenStream, MacroError> {
    let description = match args.description {
        Some(desc) => desc,
        None => extract_doc_comments(&function).ok_or_else(|| SynError(syn::Error::new(function.sig.span(), "You must provide a slash command description in either doc comments of the function or as the `description` parameter to the macro.")))?,
    };

    let name = args
        .name
        .unwrap_or_else(|| (&function.sig.ident).to_string());
    let func_name = &function.sig.ident;
    let visibility = &function.vis;
    if function.sig.asyncness.is_none() {
        return Err(SynError(syn::Error::new(
            function.sig.ident.span(),
            "Command handler must be marked as async",
        )));
    }

    let mut inner_function = function.clone();
    inner_function.sig.ident = syn::parse_quote! { inner };
    for i in &mut inner_function.sig.inputs {
        match i {
            FnArg::Receiver(_) => (),
            FnArg::Typed(t) => {
                t.attrs = vec![];
            }
        };
    }

    let parameters = params::get_args(&function)?;

    let signature = parameters.as_signature();

    let action =
        actions::create_slash_command_action(parameters.context, parameters.args.len() as u8);

    Ok(quote! {
        #visibility fn #func_name() -> ::ataraxy::Command {
            #inner_function

            ::ataraxy::Command {
                name: #name.to_string(),
                description: #description.to_string(),
                arguments: #signature,
                action: ::ataraxy::framework::command::CommandHandler(#action),
            }
        }
    }
    .into())
}
