mod actions;
mod params;

use anyhow::{Context, Result};
use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::Lit::Str;
use syn::{ItemFn, Meta};

#[derive(Default, Debug, darling::FromMeta)]
#[darling(default)]
pub struct CommandArgs {
    name: Option<String>,
    description: Option<String>,
}

fn extract_doc_comments(function: ItemFn) -> Option<String> {
    let mut doc_lines = String::new();
    for attr in function.attrs {
        if attr.path == quote::format_ident!("doc").into() {
            if let Ok(meta) = attr.parse_meta() {
                if let Meta::NameValue(nv) = meta {
                    if let Str(literal) = nv.lit {
                        let literal = literal.value();
                        let literal = literal.strip_prefix(' ').unwrap_or(&literal);

                        doc_lines += literal;
                        doc_lines += "\n";
                    }
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

pub fn command(args: CommandArgs, function: ItemFn) -> Result<TokenStream> {
    let description = match args.description {
        Some(desc) => desc,
        None => extract_doc_comments(function).ok_or(
            syn::Error::new(function.sig.span(), "You must provide a slash command description in either doc comments of the function or as the `description` parameter to the macro.")
        )?,
    };

    let name = args
        .name
        .unwrap_or_else(|| (&function.sig.ident).to_string());
    let func_name = (&function.sig.ident).to_string();
    let visibility = &function.vis;

    let mut inner_function = function.clone();
    inner_function.sig.ident = syn::parse_quote! { inner };

    return Ok(quote! {
        #visibility fn #func_name() -> ::ataraxy::Command {
            #inner_function

            ::ataraxy::Command {
                name: #func_name,
                description: #description,
                action: #action,
            }
        }
    }
    .into());
}
