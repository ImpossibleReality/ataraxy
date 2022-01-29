use serenity::model::interactions::application_command::ApplicationCommandInteractionDataOption;
use std::future::Future;
use std::pin::Pin;

pub struct CommandError {}

pub struct Command {
    name: String,
    description: String,
    action: for<'a> fn(
        super::context::SlashContext,
        &'a [ApplicationCommandInteractionDataOption],
    ) -> Pin<Box<dyn Future<Output = Result<(), CommandError>> + Send + 'a>>,
}
