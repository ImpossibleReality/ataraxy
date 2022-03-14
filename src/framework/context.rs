use serenity::model::interactions::InteractionResponseType;
use serenity::model::prelude::{Interaction, InteractionApplicationCommandCallbackDataFlags};
use serenity::prelude::Context as SerenityContext;

pub struct Context<'l> {
    responded: bool,
    serenity_context: &'l SerenityContext,
    interaction: &'l Interaction,
}

impl Context {
    pub(crate) fn new(serenity_context: &SerenityContext, interaction: &Interaction) -> Self {
        Self {
            responded: false,
            serenity_context,
            interaction,
        }
    }

    async fn reply<S: Into<String>>(&self, msg: S) {
        if self.responded {
            panic!("Already responded to the interaction")
        }

        if let Some(command) = self.interaction.application_command() {
            command
                .create_interaction_response(&self.serenity_context.http, |res| {
                    res.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|data| data.content(msg))
                })
                .await
        }
    }

    async fn reply_ephemeral<S: Into<String>>(&self, msg: S) {
        if self.responded {
            panic!("Already responded to the interaction")
        }

        if let Some(command) = self.interaction.application_command() {
            command
                .create_interaction_response(&self.serenity_context.http, |res| {
                    res.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|data| {
                            data.content(msg)
                                .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                        })
                })
                .await
        }
    }
}
