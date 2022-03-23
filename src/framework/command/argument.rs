use crate::utils::Integer;
use crate::Context;
use async_trait::async_trait;
use core::marker::Sized;
use core::option::Option;
use core::option::Option::{None, Some};
use core::result::Result;
use core::result::Result::{Err, Ok};
use serenity::futures::TryFutureExt;
use serenity::http::CacheHttp;
use serenity::model::channel::GuildChannel;
use serenity::model::id::{ChannelId, RoleId, UserId};
use serenity::model::interactions::application_command::{
    ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType as SerenityKind,
};
use serenity::model::prelude::{Channel, User};

#[derive(Debug, Copy, Clone)]
pub enum ChannelType {
    Voice,
    Text,
    Both,
}

#[derive(Debug, Clone)]
pub struct CommandArgumentOptions {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub min_len: Option<u64>,
    pub max_len: Option<u64>,
    pub channel_type: Option<ChannelType>,
}

#[derive(Debug, Clone)]
pub struct CommandArgumentSignature {
    pub name: String,
    pub description: String,
    pub argument: CommandArgumentType,
    pub options: CommandArgumentOptions,
}

#[derive(Debug, Clone)]
pub struct CommandSignature {
    /// Is context the first argument?
    pub context: bool,
    pub arguments: Vec<CommandArgumentSignature>,
}

#[derive(Debug, Clone)]
pub struct ArgumentList {
    args: Vec<CommandArgument>,
    current: usize,
}

impl ArgumentList {
    pub fn new(args: Vec<CommandArgument>) -> Self {
        Self { args, current: 0 }
    }
    pub async fn arg<A: AsCommandArgument>(&mut self, ctx: &Context) -> A {
        self.current += 1;
        match self.args.get(self.current - 1) {
            Some(v) => v.as_arg::<A>(ctx).await,
            None => {
                CommandArgument {
                    name: "".to_string(),
                    value: None,
                }
                .as_arg::<A>(ctx)
                .await
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntegerOptions {
    max: i64,
    min: i64,
}

#[derive(Debug, Clone)]
pub struct NumberOptions {
    max: f64,
    min: f64,
}

#[derive(Debug, Clone)]
pub enum CommandArgumentValueType {
    String,
    Integer(IntegerOptions),
    Number(NumberOptions),
    Channel,
    User,
    Role,
    Boolean,
}

impl CommandArgumentValueType {
    pub fn as_serenity_kind(&self) -> SerenityKind {
        match self {
            CommandArgumentValueType::String => SerenityKind::String,
            CommandArgumentValueType::Integer(_) => SerenityKind::Integer,
            CommandArgumentValueType::Number(_) => SerenityKind::Number,
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
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub enum ArgumentError {
    /// Type that came from discord is not any type that ataraxy recognizes (probably api change in discord)
    UnknownIncomingType,
    /// Type that came from discord does not match type expected for command argument
    IncorrectIncomingType(String),
    /// Argument that was not marked as optional was not provided
    IncomingArgumentNotProvided(String),
    /// Error preprocessing the argument (ie converting from a UserId to a User)
    ArgumentPreprocessingError(String),
    /// Error parsing a nested type (ie if an Option<User> failed parsing User)
    NestedParsingError(Box<Self>),
}

use ArgumentError::*;

impl CommandArgumentValue {
    pub fn to_string(self) -> String {
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

    pub fn from_resolved(
        resolved: &ApplicationCommandInteractionDataOptionValue,
    ) -> Result<Self, ArgumentError> {
        Ok(match resolved {
            ApplicationCommandInteractionDataOptionValue::String(s) => Self::String(s.clone()),
            ApplicationCommandInteractionDataOptionValue::Integer(i) => Self::Integer(*i),
            ApplicationCommandInteractionDataOptionValue::Boolean(b) => Self::Boolean(*b),
            ApplicationCommandInteractionDataOptionValue::User(u, _) => Self::User(u.id),
            ApplicationCommandInteractionDataOptionValue::Channel(c) => Self::Channel(c.id),
            ApplicationCommandInteractionDataOptionValue::Role(r) => Self::Role(r.id),
            ApplicationCommandInteractionDataOptionValue::Number(n) => Self::Number(*n),
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
    pub name: String,
    pub value: Option<CommandArgumentValue>,
}

impl CommandArgument {
    pub async fn as_arg<T: AsCommandArgument>(&self, ctx: &Context) -> T {
        T::from_returned_argument(ctx, self.value.clone())
            .await
            .expect("Error parsing argument")
    }
}

/// Represents a value that can be passed to a command, like a string, integer, or file
/// Note: This is not made for wrappers like Vec<T> or Option<T> as they should use AsCommandArgument
#[async_trait]
pub trait AsCommandArgumentValue {
    fn value_type() -> CommandArgumentValueType;
    async fn from_returned_argument(
        ctx: &Context,
        arg: Option<CommandArgumentValue>,
    ) -> Result<Self, ArgumentError>
    where
        Self: Sized;
}

/// Trait for wrappers such as Vec<T> or Option<T>
/// Auto implemented for items that impl AsCommandArgumentValue
#[async_trait]
pub trait AsCommandArgument {
    fn command_argument_type() -> CommandArgumentType;
    async fn from_returned_argument(
        ctx: &Context,
        arg: Option<CommandArgumentValue>,
    ) -> Result<Self, ArgumentError>
    where
        Self: Sized;
}

#[async_trait]
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

    async fn from_returned_argument(
        ctx: &Context,
        arg: Option<CommandArgumentValue>,
    ) -> Result<Self, ArgumentError> {
        T::from_returned_argument(ctx, arg).await
    }
}

#[async_trait]
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

    async fn from_returned_argument(
        ctx: &Context,
        arg: Option<CommandArgumentValue>,
    ) -> Result<Self, ArgumentError> {
        if let Some(arg) = arg {
            return match T::from_returned_argument(ctx, Some(arg)).await {
                Ok(arg) => Ok(Some(arg)),
                Err(e) => Err(NestedParsingError(Box::new(e))),
            };
        }
        Ok(None)
    }
}

#[async_trait]
impl AsCommandArgumentValue for String {
    fn value_type() -> CommandArgumentValueType {
        CommandArgumentValueType::String
    }

    async fn from_returned_argument(
        ctx: &Context,
        arg: Option<CommandArgumentValue>,
    ) -> Result<Self, ArgumentError> {
        if let Some(arg) = arg {
            if let CommandArgumentValue::String(arg) = arg {
                return Ok(arg.to_string());
            }
            return Err(IncorrectIncomingType(format!(
                "Expected string, found: {}",
                arg.to_string()
            )));
        }
        Err(IncomingArgumentNotProvided(
            "Required argument not provided".to_string(),
        ))
    }
}

#[async_trait]
impl<T: Integer> AsCommandArgumentValue for T {
    fn value_type() -> CommandArgumentValueType {
        CommandArgumentValueType::Integer(IntegerOptions {
            max: T::MAX as i64,
            min: T::MIN as i64,
        })
    }

    async fn from_returned_argument(
        ctx: &Context,
        arg: Option<CommandArgumentValue>,
    ) -> Result<Self, ArgumentError> {
        if let Some(arg) = arg {
            if let CommandArgumentValue::Integer(arg) = arg {
                return Ok(T::from_i64(arg));
            }
            return Err(IncorrectIncomingType(format!(
                "Expected integer, found: {}",
                arg.to_string()
            )));
        }
        Err(IncomingArgumentNotProvided(
            "Required argument not provided".to_string(),
        ))
    }
}

#[async_trait]
impl AsCommandArgumentValue for bool {
    fn value_type() -> CommandArgumentValueType {
        CommandArgumentValueType::Boolean
    }

    async fn from_returned_argument(
        ctx: &Context,
        arg: Option<CommandArgumentValue>,
    ) -> Result<Self, ArgumentError>
    where
        Self: Sized,
    {
        if let Some(arg) = arg {
            if let CommandArgumentValue::Boolean(arg) = arg {
                return Ok(arg);
            }
            return Err(IncorrectIncomingType(format!(
                "Expected boolean, found: {}",
                arg.to_string()
            )));
        }
        Err(IncomingArgumentNotProvided(
            "Required argument not provided".to_string(),
        ))
    }
}

#[async_trait]
impl AsCommandArgumentValue for UserId {
    fn value_type() -> CommandArgumentValueType {
        CommandArgumentValueType::User
    }

    async fn from_returned_argument(
        ctx: &Context,
        arg: Option<CommandArgumentValue>,
    ) -> Result<Self, ArgumentError>
    where
        Self: Sized,
    {
        if let Some(arg) = arg {
            if let CommandArgumentValue::User(arg) = arg {
                return Ok(arg);
            }
            return Err(IncorrectIncomingType(format!(
                "Expected user, found: {}",
                arg.to_string()
            )));
        }
        Err(IncomingArgumentNotProvided(
            "Required argument not provided".to_string(),
        ))
    }
}

#[async_trait]
impl AsCommandArgumentValue for User {
    fn value_type() -> CommandArgumentValueType {
        CommandArgumentValueType::User
    }

    async fn from_returned_argument(
        ctx: &Context,
        arg: Option<CommandArgumentValue>,
    ) -> Result<Self, ArgumentError>
    where
        Self: Sized,
    {
        if let Some(arg) = arg {
            if let CommandArgumentValue::User(arg) = arg {
                return arg.to_user(&ctx.http()).await.map_err(|e| {
                    ArgumentPreprocessingError("Error fetching cached user".to_string())
                });
            }
            return Err(IncorrectIncomingType(format!(
                "Expected user, found: {}",
                arg.to_string()
            )));
        }
        Err(IncomingArgumentNotProvided(
            "Required argument not provided".to_string(),
        ))
    }
}

#[async_trait]
impl AsCommandArgumentValue for ChannelId {
    fn value_type() -> CommandArgumentValueType {
        CommandArgumentValueType::Channel
    }

    async fn from_returned_argument(
        ctx: &Context,
        arg: Option<CommandArgumentValue>,
    ) -> Result<Self, ArgumentError>
    where
        Self: Sized,
    {
        if let Some(arg) = arg {
            if let CommandArgumentValue::Channel(arg) = arg {
                return Ok(arg);
            }
            return Err(IncorrectIncomingType(format!(
                "Expected channel, found: {}",
                arg.to_string()
            )));
        }
        Err(IncomingArgumentNotProvided(
            "Required argument not provided".to_string(),
        ))
    }
}

#[async_trait]
impl AsCommandArgumentValue for Channel {
    fn value_type() -> CommandArgumentValueType {
        CommandArgumentValueType::Channel
    }

    async fn from_returned_argument(
        ctx: &Context,
        arg: Option<CommandArgumentValue>,
    ) -> Result<Self, ArgumentError>
    where
        Self: Sized,
    {
        if let Some(arg) = arg {
            if let CommandArgumentValue::Channel(arg) = arg {
                return arg.to_channel(&ctx.http()).await.map_err(|e| {
                    ArgumentPreprocessingError("Error fetching cached user".to_string())
                });
            }
            return Err(IncorrectIncomingType(format!(
                "Expected channel, found: {}",
                arg.to_string()
            )));
        }
        Err(IncomingArgumentNotProvided(
            "Required argument not provided".to_string(),
        ))
    }
}

#[async_trait]
impl AsCommandArgumentValue for GuildChannel {
    fn value_type() -> CommandArgumentValueType {
        CommandArgumentValueType::Channel
    }

    async fn from_returned_argument(
        ctx: &Context,
        arg: Option<CommandArgumentValue>,
    ) -> Result<Self, ArgumentError>
    where
        Self: Sized,
    {
        if let Some(arg) = arg {
            if let CommandArgumentValue::Channel(arg) = arg {
                return arg
                    .to_channel(&ctx.http())
                    .await
                    .map_err(|e| {
                        ArgumentPreprocessingError("Error fetching cached channel".to_string())
                    })
                    .and_then(|c| match c {
                        Channel::Category(_) => Err(ArgumentPreprocessingError(
                            "Expected Guild Channel, found Channel Category".to_string(),
                        )),
                        Channel::Private(_) => Err(ArgumentPreprocessingError(
                            "Expected Guild Channel, found Private Channel".to_string(),
                        )),
                        Channel::Guild(c) => Ok(c),
                        _ => Err(ArgumentPreprocessingError(
                            "Expected Guild Channel, found Unknown Channel Type".to_string(),
                        )),
                    });
            }
            return Err(IncorrectIncomingType(format!(
                "Expected channel, found: {}",
                arg.to_string()
            )));
        }
        Err(IncomingArgumentNotProvided(
            "Required argument not provided".to_string(),
        ))
    }
}
