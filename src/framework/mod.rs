use serenity::builder::CreateApplicationCommandOption;
use serenity::model::interactions::application_command::ApplicationCommandType;
use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        id::GuildId,
        interactions::{
            application_command::{
                ApplicationCommand, ApplicationCommandInteractionDataOptionValue,
                ApplicationCommandOptionType,
            },
            Interaction, InteractionResponseType,
        },
    },
    prelude::{Context as SerenityContext, *},
};

pub mod command;
mod context;

pub use command::Command;
pub use context::Context;

/// Defines how slash commands should be created and/or merged with existing ones
pub enum CommandMergeMethod {
    /// Do not create new slash commands.
    /// May result in out-of-sync slash commands which can cause issues
    None,
    /// Reset all slash commands of bot to match ones in cache
    Set,
}

pub struct Framework {
    commands: Vec<Command>,
    command_merging: CommandMergeMethod,
}

impl Framework {
    pub fn builder() -> Self {
        Self {
            commands: Vec::new(),
            command_merging: CommandMergeMethod::Set,
        }
    }

    pub fn add_command(mut self, cmd: fn() -> Command) -> Self {
        self.commands.push(cmd());
        self
    }

    pub fn set_merge_method(mut self, method: CommandMergeMethod) -> Self {
        self.command_merging = method;
        self
    }
}

#[async_trait]
impl EventHandler for Framework {
    async fn ready(&self, ctx: SerenityContext, ready: Ready) {
        match self.command_merging {
            CommandMergeMethod::None => (),
            CommandMergeMethod::Set => {
                let guild_id = GuildId(782428786229903380);

                let commands = guild_id
                    .set_application_commands(&ctx.http, |commands| {
                        let mut cmds = commands;
                        for command in &self.commands {
                            cmds = cmds.create_application_command(|cmd| {
                                cmd.name(&command.name)
                                    .description(&command.description)
                                    .kind(ApplicationCommandType::ChatInput)
                                    .set_options(
                                        command
                                            .arguments
                                            .arguments
                                            .iter()
                                            .map(|opt| {
                                                CreateApplicationCommandOption::default()
                                                    .name(&opt.name)
                                                    .description(&opt.description)
                                                    .required(!opt.argument.optional)
                                                    .kind(
                                                        opt.argument.value_type.as_serenity_kind(),
                                                    )
                                                    .clone()
                                            })
                                            .collect(),
                                    )
                            })
                        }
                        cmds
                    })
                    .await;
            }
        }
    }
}
