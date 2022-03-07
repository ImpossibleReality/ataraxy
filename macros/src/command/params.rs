use anyhow::Result;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::FnArg::Typed;
use syn::{FnArg, ItemFn, Pat, Type};

pub struct CommandArg {
    name: String,
    description: Option<String>,
    span: Span,
    ty: Type,
}

pub struct CommandParameters {
    pub args: Vec<CommandArg>,
    pub context: bool,
}

pub fn get_args(func: &ItemFn) -> Result<CommandParameters, syn::Error> {
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
                return Err(syn::Error::new(
                    arg.span(),
                    "Cannot have a self argument in a command",
                ))
            }
            Typed(t) => {
                // TODO: Make context more robust
                if let Type::Path(p) = &*t.ty {
                    if p.path.is_ident("Context") {
                        if context {
                            return Err(syn::Error::new(
                                arg.span(),
                                "Cannot have multiple context arguments in a command",
                            ));
                        }
                        if i != 0 {
                            return Err(syn::Error::new(
                                arg.span(),
                                "Context must be first argument in a command",
                            ));
                        }
                        context = true;
                        continue;
                    }
                }
                if let Pat::Ident(id) = &*t.pat {
                    args.push(CommandArg {
                        name: id.ident.to_string(),
                        description: None,
                        span: t.span(),
                        ty: *t.ty.clone(),
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
            .map(|arg| arg.clone().as_signature())
            .collect();

        quote! {
            ::ataraxy::framework::command::CommandSignature {
                context: #context,
                arguments: Vec::from([#(#args),*])
            }
        }
    }
}

impl CommandArg {
    pub fn as_signature(&self) -> TokenStream {
        let ty = &self.ty;
        let name = &self.name;
        let description = self.description.as_ref().unwrap_or(&self.name);
        quote_spanned! { self.span =>
            ::ataraxy::framework::command::CommandArgumentSignature {
                name: #name.to_string(),
                description: #description.to_string(),
                argument: <#ty as ::ataraxy::framework::command::AsCommandArgument>::command_argument_type(),
            }
        }
    }
}
