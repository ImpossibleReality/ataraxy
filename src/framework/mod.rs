use ataraxy_macros::command_ide_arg_support;
use serenity::builder::{CreateApplicationCommandOption, CreateApplicationCommands};

use serenity::model::interactions::application_command::ApplicationCommandType;
use serenity::model::interactions::InteractionType;

use serenity::{
    async_trait,
    model::{gateway::Ready, id::GuildId, interactions::Interaction},
    prelude::{Context as SerenityContext, *},
};

use serenity::http::CacheHttp;
use serenity::model::prelude::application_command::{
    ApplicationCommand, ApplicationCommandOptionType,
};
use std::collections::HashMap;

pub mod command;
mod context;

use crate::framework::command::argument::{ArgumentList, CommandArgument, CommandArgumentValue};
pub use command::Command;
pub use context::Context;

/// Defines how slash commands should be created and/or merged with existing ones
#[derive(Clone, Debug)]
pub enum CommandMergeMethod {
    /// Do not create new slash commands.
    /// May result in out-of-sync slash commands which can cause issues
    None,
    /// Reset all slash commands of bot to match ones in cache
    Set,
}

#[derive(Clone, Debug)]
pub struct Framework {
    commands: HashMap<String, Vec<ValidCommand>>,
    command_merging: CommandMergeMethod,
}

pub trait IntoValidCommand {
    fn into_command(self) -> ValidCommand;
}

impl IntoValidCommand for Command {
    fn into_command(self) -> ValidCommand {
        ValidCommand::Command(self)
    }
}

impl IntoValidCommand for SubCommands {
    fn into_command(self) -> ValidCommand {
        ValidCommand::SubCommands(self)
    }
}

impl<T: Fn() -> C, C: IntoValidCommand> IntoValidCommand for T {
    fn into_command(self) -> ValidCommand {
        self().into_command()
    }
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
    // Macro is for IDE support, since CLion freaks out without it
    // What is weird is that SubCommand::command() doesn't freak aut about
    // the function signature change for some reason ¯\_(ツ)_/¯
    #[command_ide_arg_support]
    pub fn command<T: Any>(mut self, cmd: T) -> Self {
        let command = cmd.into_command();
        self.commands
            .entry(command.name().clone())
            .or_insert_with(Vec::new)
            .push(command);
        self
    }

    pub fn set_merge_method(mut self, method: CommandMergeMethod) -> Self {
        self.command_merging = method;
        self
    }
}

impl Default for Framework {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a list of subcommands as a command
#[derive(Clone, Debug)]
pub struct SubCommands {
    pub name: String,
    pub description: String,
    pub guilds: Option<Vec<u64>>,
    pub subcommands: HashMap<String, SubCommand>,
}

impl SubCommands {
    /// Create a new command with no subcommands
    pub fn new<S: Into<String>>(name: S, description: S) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            guilds: None,
            subcommands: HashMap::new(),
        }
    }

    /// Add a subcommand to the command
    pub fn command(mut self, cmd: fn() -> Command) -> Self {
        let cmd = cmd();
        self.subcommands
            .insert(cmd.name.clone(), SubCommand::SubCommand(cmd));
        self
    }

    /// Add a command group to the command
    pub fn group(mut self, group: CommandGroup) -> Self {
        self.subcommands
            .insert(group.name.clone(), SubCommand::SubCommandGroup(group));
        self
    }

    pub fn guild(mut self, guild: u64) -> Self {
        match &mut self.guilds {
            None => self.guilds = Some(vec![guild]),
            Some(guilds) => guilds.push(guild),
        }
        self
    }
}

#[derive(Clone, Debug)]
pub struct CommandGroup {
    pub name: String,
    pub description: String,
    pub subcommands: HashMap<String, Command>,
}
impl CommandGroup {
    /// Create a new command group
    pub fn new<T: IntoIterator, S: Into<String>>(name: S, description: S, commands: T) -> Self
    where
        T::Item: Fn() -> Command,
    {
        Self {
            name: name.into(),
            description: description.into(),
            subcommands: commands
                .into_iter()
                .map(|c| {
                    let cmd = c();
                    (cmd.name.clone(), cmd)
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SubCommand {
    SubCommand(Command),
    SubCommandGroup(CommandGroup),
}

/// Represents any kind of valid command (either simple command of command with subcommands)
#[derive(Clone, Debug)]
pub enum ValidCommand {
    Command(Command),
    SubCommands(SubCommands),
}

impl ValidCommand {
    pub fn name(&self) -> &String {
        match self {
            ValidCommand::Command(cmd) => &cmd.name,
            ValidCommand::SubCommands(subcmds) => &subcmds.name,
        }
    }

    pub fn description(&self) -> &String {
        match self {
            ValidCommand::Command(cmd) => &cmd.description,
            ValidCommand::SubCommands(subcmds) => &subcmds.description,
        }
    }

    pub fn guilds(&self) -> &Option<Vec<u64>> {
        match self {
            ValidCommand::Command(c) => &c.guilds,
            ValidCommand::SubCommands(sc) => &sc.guilds,
        }
    }
}

fn create_command(command: &ValidCommand, cmds: &mut CreateApplicationCommands) {
    match command {
        ValidCommand::Command(command) => {
            cmds.create_application_command(|cmd| {
                cmd.name(&command.name)
                    .description(&command.description)
                    .kind(ApplicationCommandType::ChatInput)
                    .set_options(
                        command
                            .arguments
                            .arguments
                            .iter()
                            .map(|opt| opt.as_serenity_option())
                            .collect(),
                    )
            });
        }
        ValidCommand::SubCommands(subcommands) => {
            cmds.create_application_command(|cmd| {
                cmd.name(&subcommands.name)
                    .description(&subcommands.description)
                    .kind(ApplicationCommandType::ChatInput)
                    .set_options(
                        subcommands
                            .subcommands
                            .values()
                            .map(|subcommand| match subcommand {
                                SubCommand::SubCommand(subcmd) => {
                                    let mut c = CreateApplicationCommandOption::default();
                                    c.kind(ApplicationCommandOptionType::SubCommand)
                                        .name(&subcmd.name)
                                        .description(&subcmd.description);
                                    for arg in &subcmd.arguments.arguments {
                                        c.add_sub_option(arg.as_serenity_option());
                                    }
                                    c
                                }
                                SubCommand::SubCommandGroup(subcmdgroup) => {
                                    let mut c = CreateApplicationCommandOption::default()
                                        .kind(ApplicationCommandOptionType::SubCommandGroup)
                                        .name(&subcmdgroup.name)
                                        .description(&subcmdgroup.description)
                                        .clone();
                                    for subcmd in subcmdgroup.subcommands.values() {
                                        c.create_sub_option(|c| {
                                            c.kind(ApplicationCommandOptionType::SubCommand)
                                                .name(&subcmd.name)
                                                .description(&subcmd.description);
                                            for arg in &subcmd.arguments.arguments {
                                                c.add_sub_option(arg.as_serenity_option());
                                            }
                                            c
                                        });
                                    }
                                    c
                                }
                            })
                            .collect(),
                    )
            });
        }
    }
}

#[async_trait]
impl EventHandler for Framework {
    async fn ready(&self, ctx: SerenityContext, _ready: Ready) {
        match self.command_merging {
            CommandMergeMethod::None => (),
            CommandMergeMethod::Set => {
                let global_commands: Vec<&ValidCommand> = self
                    .commands
                    .values()
                    .flatten()
                    .filter(|c| c.guilds().is_none())
                    .collect();

                let mut guild_commands: HashMap<u64, Vec<ValidCommand>> = HashMap::new();

                for c in self.commands.values().flatten() {
                    if let Some(guilds) = c.guilds() {
                        for guild in guilds {
                            guild_commands
                                .entry(*guild)
                                .or_insert_with(Vec::new)
                                .push(c.clone());
                        }
                    }
                }

                ApplicationCommand::set_global_application_commands(&ctx.http(), |c| {
                    for cmd in global_commands {
                        create_command(cmd, c);
                    }
                    c
                })
                .await
                .unwrap();

                for (guild, commands) in guild_commands {
                    let guild = GuildId(guild);

                    guild
                        .set_application_commands(&ctx.http(), |c| {
                            for cmd in commands {
                                create_command(&cmd, c);
                            }
                            c
                        })
                        .await
                        .unwrap();
                }
            }
        }
    }

    async fn interaction_create(&self, ctx: SerenityContext, interaction: Interaction) {
        match interaction.kind() {
            InteractionType::ApplicationCommand => {
                let interaction_command = interaction.application_command().unwrap();
                let name = &interaction_command.data.name;
                if let Some(commands) = self.commands.get(&*name) {
                    let mut command = None;

                    let possible_commands: Vec<&ValidCommand> = commands
                        .iter()
                        .filter(|c| match c.guilds() {
                            Some(guilds) => {
                                if let Some(guild) = &interaction_command.guild_id {
                                    guilds.contains(&guild.0)
                                } else {
                                    false
                                }
                            }
                            None => interaction_command.guild_id.is_none(),
                        })
                        .collect();

                    if possible_commands.len() == 1 {
                        command = possible_commands.get(0).map(|c| *c)
                    } else if possible_commands.len() == 0 {
                        command = commands.iter().filter(|c| c.guilds().is_none()).next()
                    }

                    match command.expect("Error finding command.") {
                        ValidCommand::Command(command) => {
                            let context = Context::new(&ctx, &interaction_command);

                            let options = interaction_command.data.options;

                            let args: Result<Vec<CommandArgument>, _> = options
                                .iter()
                                .map(|opt| {
                                    let value = match &opt.resolved {
                                        Some(arg) => match CommandArgumentValue::from_resolved(arg)
                                        {
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
                            if args.is_err() {
                                return;
                            }
                            command.action.0(context, &ArgumentList::new(args.unwrap())).await;
                        }
                        ValidCommand::SubCommands(subcmds) => {
                            let options = &interaction_command.data.options;
                            if let Some(sub_cmd_opt) = options.get(0) {
                                if let Some(called_sub_cmd) =
                                    subcmds.subcommands.get(&*sub_cmd_opt.name)
                                {
                                    match called_sub_cmd {
                                        SubCommand::SubCommand(subcmd) => {
                                            let context = Context::new(&ctx, &interaction_command);
                                            let new_options = sub_cmd_opt.options.clone();

                                            let args: Result<Vec<CommandArgument>, _> = new_options
                                                .iter()
                                                .map(|opt| {
                                                    let value = match &opt.resolved {
                                                        Some(arg) => match CommandArgumentValue::from_resolved(arg)
                                                        {
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

                                            if args.is_err() {
                                                return;
                                            }

                                            subcmd.action.0(
                                                context,
                                                &ArgumentList::new(args.unwrap()),
                                            )
                                            .await;
                                        }
                                        SubCommand::SubCommandGroup(subcmdgroup) => {
                                            if let Some(sub_cmd_opt) = sub_cmd_opt.options.get(0) {
                                                if let Some(subcmd) =
                                                    subcmdgroup.subcommands.get(&*sub_cmd_opt.name)
                                                {
                                                    let context =
                                                        Context::new(&ctx, &interaction_command);
                                                    let new_options = sub_cmd_opt.options.clone();

                                                    let args: Result<Vec<CommandArgument>, _> = new_options
                                                        .iter()
                                                        .map(|opt| {
                                                            let value = match &opt.resolved {
                                                                Some(arg) => match CommandArgumentValue::from_resolved(arg)
                                                                {
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

                                                    if args.is_err() {
                                                        return;
                                                    }

                                                    subcmd.action.0(
                                                        context,
                                                        &ArgumentList::new(args.unwrap()),
                                                    )
                                                    .await;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                return;
            }
            _ => return,
        }
    }
}
