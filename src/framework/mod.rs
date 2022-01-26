use serenity::prelude::Context;
mod command;
mod context;
mod handler;

pub struct Command<F: Fn> {
    handler: F,
}

pub struct Framework {
    handler: Vec<EventHandler>,
    commands: Vec<command::Command>,
}
