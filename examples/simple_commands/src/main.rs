use ataraxy::command;
use ataraxy::CommandGroup;
use ataraxy::Context;
use ataraxy::Framework;
use ataraxy::SubCommands;
use dotenv::dotenv;
use serenity::model::channel::Channel;
use serenity::model::prelude::User;
use serenity::Client;
use std::env;

/// Says "Hello world" Version 2
#[command(guild(782428786229903380))]
async fn say_hello(
    ctx: Context,
    #[option(channel_type = "text", description = "Text channel to say hello to")] channel: Channel,
) {
    channel
        .id()
        .send_message(&ctx.http(), |m| m.content("Hello, world!"))
        .await;
    ctx.reply_ephemeral("Sent message").await;
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let application_id = env::var("APPLICATION_ID")
        .expect("Expected app id in the environment")
        .parse()
        .unwrap();

    let framework = Framework::new().command(say_hello);

    let mut client = Client::builder(token)
        .event_handler(framework)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
