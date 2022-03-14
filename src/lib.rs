//! A Discord slash command framework for Serenity
//! # Usage
//! ```rust
//!
//! ```

pub mod framework;

pub use ataraxy_macros::command;
pub use framework::Command;
pub use framework::Context;
pub use framework::Framework;
use serenity::model::prelude::Embed;
