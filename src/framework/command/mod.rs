pub mod argument;

use crate::framework::command::argument::{ArgumentList, CommandSignature};

use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct CommandError {}

type Handler = for<'a> fn(
    super::context::Context,
    &'a ArgumentList,
) -> Pin<Box<dyn Future<Output = ()> + Send>>;

#[derive(Clone)]
pub struct CommandHandler(pub Handler);

impl Debug for CommandHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Command Handler")
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub guilds: Option<Vec<u64>>,
    pub arguments: CommandSignature,
    pub action: CommandHandler,
}
