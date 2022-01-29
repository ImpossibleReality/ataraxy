use serenity::model::interactions::application_command::ApplicationCommandInteractionDataOption;
use std::future::Future;
use std::pin::Pin;

pub enum CommandArgumentType {
    String,
    Integer,
    Number,
}

pub struct CommandArgument {
    optional: bool,
    argument_type: CommandArgumentType,
}

pub enum ReturnedCommandArgument {
    Optional(Option<ReturnedCommandArgument>),
    String(String),
    Integer(i64),
    Number(f64),
}

pub trait AsCommandArgument {
    fn command_argument() -> CommandArgument;
    fn from_returned_argument() -> Self;
}

impl<T> AsCommandArgument for Option<T>
where
    T: AsCommandArgument,
{
    fn command_argument() -> CommandArgument {
        CommandArgument {
            optional: true,
            argument_type: T.command_argument(),
        }
    }

    fn from_returned_argument() -> Self {
        todo!()
    }
}

pub struct CommandError {}

pub struct Command {
    name: String,
    description: String,
    action: for<'a> fn(
        super::context::SlashContext,
        &'a [ApplicationCommandInteractionDataOption],
    ) -> Pin<Box<dyn Future<Output = Result<(), CommandError>> + Send + 'a>>,
}
