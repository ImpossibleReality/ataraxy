use serenity::model::id::{ChannelId, RoleId, UserId};
use serenity::model::interactions::application_command::{
    ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType as SerenityKind,
};
use serenity::model::{channel::Channel, guild::Role, user::User};
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone)]
pub enum CommandArgumentValueType {
    String,
    Integer,
    Number,
    Channel,
    User,
    Role,
    Boolean,
}

impl CommandArgumentValueType {
    pub fn as_serenity_kind(&self) -> SerenityKind {
        match self {
            CommandArgumentValueType::String => SerenityKind::String,
            CommandArgumentValueType::Integer => SerenityKind::Integer,
            CommandArgumentValueType::Number => SerenityKind::Number,
            CommandArgumentValueType::Channel => SerenityKind::Channel,
            CommandArgumentValueType::User => SerenityKind::User,
            CommandArgumentValueType::Role => SerenityKind::Role,
            CommandArgumentValueType::Boolean => SerenityKind::Boolean,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CommandArgumentValue {
    String(String),
    Integer(i64),
    Number(f64),
    Channel(ChannelId),
    User(UserId),
    Role(RoleId),
    Boolean(boolId),
}

enum ArgumentError {
    UnknownIncomingType,
}

impl CommandArgumentValue {
    fn to_string(self) -> String {
        match self {
            Self::String(_) => "String",
            Self::Integer(_) => "Integer",
            Self::Number(_) => "Float",
            CommandArgumentValue::Channel(_) => "Channel",
            CommandArgumentValue::User(_) => "User",
            CommandArgumentValue::Role(_) => "Role",
            CommandArgumentValue::Boolean(_) => "Boolean",
        }
        .to_string()
    }

    fn from_resolved(
        resolved: ApplicationCommandInteractionDataOptionValue,
    ) -> Result<Self, ArgumentError> {
        Ok(match resolved {
            ApplicationCommandInteractionDataOptionValue::String(s) => Self::String(s),
            ApplicationCommandInteractionDataOptionValue::Integer(i) => Self::Integer(i),
            ApplicationCommandInteractionDataOptionValue::Boolean(b) => Self::Boolean(b),
            ApplicationCommandInteractionDataOptionValue::User(u, _) => Self::User(u.id),
            ApplicationCommandInteractionDataOptionValue::Channel(c) => Self::Channel(c.id),
            ApplicationCommandInteractionDataOptionValue::Role(r) => Self::Role(r.id),
            ApplicationCommandInteractionDataOptionValue::Number(n) => Self::Number(n),
            _ => return Err(ArgumentError::UnknownIncomingType),
        })
    }
}

#[derive(Debug, Clone)]
pub struct CommandArgumentType {
    pub optional: bool,
    pub value_type: CommandArgumentValueType,
}

#[derive(Debug, Clone)]
pub struct CommandArgument {
    name: String,
    value: Option<CommandArgumentValue>,
}

impl CommandArgument {
    pub fn as_arg<T: AsCommandArgument>(&self) -> T {
        T::from_returned_argument(self.value.clone()).expect("Error parsing argument")
    }
}

/// Represents a value that can be passed to a command, like a string, integer, or file
/// Note: This is not made for wrappers like Vec<T> or Option<T> as they should use AsCommandArgument
pub trait AsCommandArgumentValue {
    fn value_type() -> CommandArgumentValueType;
    fn from_returned_argument(arg: Option<CommandArgumentValue>) -> Result<Self, String>
    where
        Self: Sized;
}

/// Trait for wrappers such as Vec<T> or Option<T>
/// Auto implemented for items that impl AsCommandArgumentValue
pub trait AsCommandArgument {
    fn command_argument_type() -> CommandArgumentType;
    fn from_returned_argument(arg: Option<CommandArgumentValue>) -> Result<Self, String>
    where
        Self: Sized;
}

impl<T> AsCommandArgument for T
where
    T: AsCommandArgumentValue,
{
    fn command_argument_type() -> CommandArgumentType {
        CommandArgumentType {
            optional: false,
            value_type: T::value_type(),
        }
    }

    fn from_returned_argument(arg: Option<CommandArgumentValue>) -> Result<Self, String> {
        T::from_returned_argument(arg)
    }
}

impl<T> AsCommandArgument for Option<T>
where
    T: AsCommandArgumentValue,
{
    fn command_argument_type() -> CommandArgumentType {
        CommandArgumentType {
            optional: true,
            value_type: T::value_type(),
        }
    }

    fn from_returned_argument(arg: Option<CommandArgumentValue>) -> Result<Self, String> {
        if let Some(arg) = arg {
            return match T::from_returned_argument(Some(arg)) {
                Ok(arg) => Ok(Some(arg)),
                Err(e) => Err(format!("Error parsing option: {}", e)),
            };
        }
        Ok(None)
    }
}

impl AsCommandArgumentValue for String {
    fn value_type() -> CommandArgumentValueType {
        CommandArgumentValueType::String
    }

    fn from_returned_argument(arg: Option<CommandArgumentValue>) -> Result<Self, String> {
        if let Some(arg) = arg {
            if let CommandArgumentValue::String(arg) = arg {
                return Ok(arg);
            }
            return Err(format!("Expected string, found: {}", arg.to_string()));
        }
        Err("Required argument not provided".to_string())
    }
}

#[derive(Debug, Clone)]
pub struct CommandError {}

#[derive(Debug, Clone)]
pub struct CommandArgumentSignature {
    pub name: String,
    pub description: String,
    pub argument: CommandArgumentType,
}

#[derive(Debug, Clone)]
pub struct CommandSignature {
    /// Is context the first argument?
    pub context: bool,
    pub arguments: Vec<CommandArgumentSignature>,
}

#[derive(Clone)]
pub struct CommandHandler(
    pub  for<'a> fn(
        super::context::Context,
        &'a [CommandArgument],
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
