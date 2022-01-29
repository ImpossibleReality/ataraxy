use serenity::prelude::Context;
use serenity::prelude::EventHandler;
use ataraxy_macros;

mod command;
mod context;
mod handler;

pub struct Command<F: Fn() -> ()> {
    handler: F,
}

pub struct Framework {
    handler: Vec<Box<EventHandler>>,
    commands: Vec<command::Command>,
}
