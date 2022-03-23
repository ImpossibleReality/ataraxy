pub mod argument;

use crate::framework::command::argument::{ArgumentList, CommandSignature};
use crate::utils::Integer;
use num_traits::AsPrimitive;
use serenity::model::id::{ChannelId, RoleId, UserId};
use serenity::model::interactions::application_command::{
    ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType as SerenityKind,
};
use serenity::model::{channel::Channel, guild::Role, user::User};
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct CommandError {}

#[derive(Clone)]
pub struct CommandHandler(
    pub  for<'a> fn(
        super::context::Context,
        &'a ArgumentList,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>>,
);

impl Debug for CommandHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Command Handler")
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub arguments: CommandSignature,
    pub action: CommandHandler,
}
