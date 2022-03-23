use crate::Command;
use serenity::http::{CacheHttp, Http};
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::interactions::InteractionResponseType;
use serenity::model::prelude::InteractionApplicationCommandCallbackDataFlags;
use serenity::prelude::Context as SerenityContext;
use std::sync::Arc;

#[derive(Clone)]
pub struct Context {
    responded: bool,
    serenity_context: SerenityContext,
    interaction: ApplicationCommandInteraction,
}

impl Context {
    pub fn new(
        serenity_context: &SerenityContext,
        interaction: &ApplicationCommandInteraction,
    ) -> Self {
        Self {
            responded: false,
            serenity_context: serenity_context.clone(),
            interaction: interaction.clone(),
        }
    }

    pub fn http(&self) -> &Http {
        self.serenity_context.http()
    }

    pub async fn reply<S: Into<String>>(&self, msg: S) {
        if self.responded {
            panic!("Already responded to the interaction")
        }

        self.interaction
            .create_interaction_response(&self.serenity_context.http, |res| {
                res.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|data| data.content(msg.into()))
            })
            .await;
    }

    pub async fn reply_ephemeral<S: Into<String>>(&self, msg: S) {
        if self.responded {
            panic!("Already responded to the interaction")
        }

        self.interaction
            .create_interaction_response(&self.serenity_context.http, |res| {
                res.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|data| {
                        data.content(msg.into())
                            .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                    })
            })
            .await;
    }
}
