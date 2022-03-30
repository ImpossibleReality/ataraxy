use crate::utils::MacroError::*;
use crate::utils::{quote_option, quote_vec, MacroError, Multiple};

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::FnArg::Typed;
use syn::{FnArg, ItemFn, Meta, Pat, Type};

#[derive(Debug, Clone, darling::FromMeta)]
enum ChannelType {
    /// All voice channels (but not stage channels)
    Voice,
    /// Only stage channels
    Stage,
    /// Any text channel, including news channels and normal text channels. No threads.
    Text,
    /// Only news channels
    News,
    /// Private threads in news channels
    NewsThreads,
    /// Any public threads
    Thread,
    /// Any channel category
    Category,
    /// Private channels (ie DMs)
    Private,
    /// Any type of channel
    All,
}

#[derive(Default, Debug, darling::FromMeta)]
#[darling(default)]
pub struct CommandArgOptions {
    min: Option<f64>,
    max: Option<f64>,
    min_len: Option<u64>,
    max_len: Option<u64>,
    channel_type: Option<Multiple<ChannelType>>,
    name: Option<String>,
    description: Option<String>,
}

pub struct CommandArg {
    name: String,
    span: Span,
    ty: Type,
    options: CommandArgOptions,
}

pub struct CommandParameters {
    pub args: Vec<CommandArg>,
    pub context: bool,
}

pub fn get_args(func: &ItemFn) -> Result<CommandParameters, MacroError> {
    if func.sig.inputs.is_empty() {
        return Ok(CommandParameters {
            args: Vec::new(),
            context: false,
        });
    }

    let mut args = Vec::new();
    let mut context = false;

    for (i, arg) in func.sig.inputs.iter().enumerate() {
        match arg {
            FnArg::Receiver(_) => {
                return Err(SynError(syn::Error::new(
                    arg.span(),
                    "Cannot have a self argument in a command",
                )))
            }
            Typed(t) => {
                // TODO: Make context more robust
                if let Type::Path(p) = &*t.ty {
                    if p.path.is_ident("Context") {
                        if context {
                            return Err(SynError(syn::Error::new(
                                arg.span(),
                                "Cannot have multiple context arguments in a command",
                            )));
                        }
                        if i != 0 {
                            return Err(SynError(syn::Error::new(
                                arg.span(),
                                "Context must be first argument in a command",
                            )));
                        }
                        context = true;
                        continue;
                    }
                }

                let options = <CommandArgOptions as darling::FromMeta>::from_list(
                    &t.attrs
                        .iter()
                        .filter_map(|a| {
                            a.parse_meta().ok().and_then(|m| {
                                if let Meta::List(l) = m {
                                    if l.path.is_ident("option") {
                                        Some(l.nested)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            })
                        })
                        .flatten()
                        .collect::<Vec<_>>(),
                )
                .map_err(DarlingError)?;

                if let Pat::Ident(id) = &*t.pat {
                    args.push(CommandArg {
                        name: id.ident.to_string(),
                        span: t.span(),
                        ty: *t.ty.clone(),
                        options,
                    })
                }
            }
        }
    }
    Ok(CommandParameters { args, context })
}

impl CommandParameters {
    pub fn as_signature(&self) -> TokenStream {
        let context = self.context;

        let args: Vec<TokenStream> = self
            .args
            .iter()
            .map(|arg| arg.as_signature())
            .collect();

        quote! {
            ::ataraxy::framework::command::argument::CommandSignature {
                context: #context,
                arguments: Vec::from([#(#args),*])
            }
        }
    }
}

impl CommandArg {
    pub fn as_signature(&self) -> TokenStream {
        let ty = &self.ty;
        let name = self.options.name.as_ref().unwrap_or(&self.name);
        let description = self.options.description.as_ref().unwrap_or(name);
        let options = self.options_as_tokens();
        quote_spanned! { self.span =>
            ::ataraxy::framework::command::argument::CommandArgumentSignature {
                name: #name.to_string(),
                description: #description.to_string(),
                argument: <#ty as ::ataraxy::framework::command::argument::AsCommandArgument>::command_argument_type(),
                options: #options
            }
        }
    }

    fn options_as_tokens(&self) -> TokenStream {
        let min = quote_option(&self.options.min);
        let max = quote_option(&self.options.max);
        let min_len = quote_option(&self.options.min_len);
        let max_len = quote_option(&self.options.max_len);
        let channel_type = quote_option(&self.options.channel_type.clone().map(|cs| {
            quote_vec(&cs.0.iter().map(|c| match c {
                ChannelType::Text => {
                    quote! { ::ataraxy::framework::command::argument::ChannelType::Text }
                }
                ChannelType::Voice => {
                    quote! { ::ataraxy::framework::command::argument::ChannelType::Voice }
                }
                ChannelType::All => {
                    quote! { ::ataraxy::framework::command::argument::ChannelType::All }
                }
                ChannelType::Category => {
                    quote! { ::ataraxy::framework::command::argument::ChannelType::Category }
                }
                ChannelType::News => {
                    quote! { ::ataraxy::framework::command::argument::ChannelType::News }
                }
                ChannelType::NewsThreads => {
                    quote! { ::ataraxy::framework::command::argument::ChannelType::NewsThreads }
                }
                ChannelType::Private => {
                    quote! { ::ataraxy::framework::command::argument::ChannelType::Private }
                }
                ChannelType::Stage => {
                    quote! { ::ataraxy::framework::command::argument::ChannelType::Stage }
                }
                ChannelType::Thread => {
                    quote! { ::ataraxy::framework::command::argument::ChannelType::Thread }
                }
            }).collect::<Vec<TokenStream>>())
        }));
        quote! {
            ::ataraxy::framework::command::argument::CommandArgumentOptions {
                min: #min,
                max: #max,
                min_len: #min_len,
                max_len: #max_len,
                channel_type: #channel_type,
            }
        }
    }
}
