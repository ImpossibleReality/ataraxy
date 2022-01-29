use anyhow::Result;
use syn::spanned::Spanned;
use syn::FnArg::Typed;
use syn::{ItemFn, Path};

enum CommandArgType {
    String,
    Integer,
    Number,
    Boolean,
}

struct CommandArgs {
    optional: bool,
    param_type: CommandArgType,
    name: String,
    description: String,
}

struct CommandParameters {
    args: Vec<CommandArgs>,
    context: bool,
}

fn get_args(func: ItemFn) -> Result<CommandParameters, syn::Error> {
    if func.sig.inputs.is_empty() {
        return Ok(CommandParameters {
            args: Vec::new(),
            context: false,
        });
    }

    let mut args = Vec::new();
    let mut context = false;

    for arg in func.sig.inputs.iter() {
        match arg {
            Reciever(_) => {
                return Err(syn::Error::new(
                    arg.span(),
                    "Cannot have a self argument in a command",
                ))
            }
            Typed(t) => match t {
                Path(p) => {
                    if p.path.is_ident("Context") {
                        if context {
                            return Err(syn::Error::new(
                                arg.span(),
                                "Cannot have multiple context arguments in a command",
                            ));
                        }
                        context = true;
                    } else {
                        return Err(syn::Error::new(
                            arg.span(),
                            "Cannot have a non-Context argument in a command",
                        ));
                    }
                }
                _ => return Err(syn::Error::new(arg.span(), "Unknown type in a command")),
            },
        }
    }
    Ok(CommandParameters { args, context })
}
