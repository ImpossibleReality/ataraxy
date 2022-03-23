use ataraxy_macros::command_ide_arg_support;
use serenity::builder::CreateApplicationCommandOption;
use serenity::client::bridge::gateway::event::ShardStageUpdateEvent;
use serenity::model::channel::{
    Channel, ChannelCategory, GuildChannel, Message, PartialGuildChannel, Reaction, StageInstance,
};
use serenity::model::event::{
    ChannelPinsUpdateEvent, GuildMemberUpdateEvent, GuildMembersChunkEvent, InviteCreateEvent,
    InviteDeleteEvent, MessageUpdateEvent, PresenceUpdateEvent, ResumedEvent, ThreadListSyncEvent,
    ThreadMembersUpdateEvent, TypingStartEvent, VoiceServerUpdateEvent,
};
use serenity::model::gateway::Presence;
use serenity::model::guild::{
    Emoji, Guild, GuildUnavailable, Integration, Member, PartialGuild, Role, ThreadMember,
};
use serenity::model::id::{ApplicationId, ChannelId, EmojiId, IntegrationId, MessageId, RoleId};
use serenity::model::interactions::application_command::ApplicationCommandType;
use serenity::model::interactions::InteractionType;
use serenity::model::prelude::{CurrentUser, User, VoiceState};
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
use std::any::Any;
use std::collections::HashMap;

pub mod command;
mod context;

use crate::framework::command::argument::{ArgumentList, CommandArgument, CommandArgumentValue};
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
    commands: HashMap<String, Command>,
    command_merging: CommandMergeMethod,
}

impl Framework {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            command_merging: CommandMergeMethod::Set,
        }
    }
    /// Adds a command to the framework
    /// see [command!] for more details
    #[command_ide_arg_support]
    pub fn command<T: Any>(mut self, cmd: T) -> Self {
        self.commands.insert(cmd().name, cmd());
        self
    }

    pub fn set_merge_method(mut self, method: CommandMergeMethod) -> Self {
        self.command_merging = method;
        self
    }
}

#[async_trait]
impl EventHandler for Framework {
    async fn ready(&self, ctx: SerenityContext, _ready: Ready) {
        match self.command_merging {
            CommandMergeMethod::None => (),
            CommandMergeMethod::Set => {
                let guild_id = GuildId(782428786229903380);

                let _commands = guild_id
                    .set_application_commands(&ctx.http, |commands| {
                        let mut cmds = commands;
                        for (_, command) in &self.commands {
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

    async fn interaction_create(&self, ctx: SerenityContext, interaction: Interaction) {
        match interaction.kind() {
            InteractionType::ApplicationCommand => {
                let interaction_command = interaction.application_command().unwrap();
                let name = &interaction_command.data.name;
                if let Some(command) = self.commands.get(&*name) {
                    let context = Context::new(&ctx, &interaction_command);

                    let options = interaction_command.data.options;

                    let args: Result<Vec<CommandArgument>, _> = options
                        .iter()
                        .map(|opt| {
                            let value = match &opt.resolved {
                                Some(arg) => match CommandArgumentValue::from_resolved(arg) {
                                    Ok(v) => Some(v),
                                    Err(e) => return Err(e),
                                },
                                None => None,
                            };
                            Ok(CommandArgument {
                                name: opt.name.clone(),
                                value,
                            })
                        })
                        .collect();
                    if let Err(_) = args {
                        return;
                    }
                    command.action.0(context, &ArgumentList::new(args.unwrap())).await;
                }
                return;
            }
            _ => return,
        }
    }
}
