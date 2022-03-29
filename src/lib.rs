//! A Discord slash command framework for Serenity
//! # Usage
//! ```rust
//!
//! ```

pub mod framework;
mod numbers;
mod utils;

pub use ataraxy_macros::command;
pub use framework::Command;
pub use framework::CommandGroup;
pub use framework::Context;
pub use framework::Framework;
pub use framework::SubCommands;
