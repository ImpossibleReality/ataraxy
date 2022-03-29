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

/// Says "Hello world"
#[command]
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

/// Says "Hello world"
#[command]
async fn other_say_hello(
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
    let test_cmd = SubCommands::new("testing", "Amazing test thingy")
        .command(say_hello)
        .group(CommandGroup::new(
            "others",
            "Do other stuff",
            [other_say_hello],
        ));

    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let application_id = env::var("APPLICATION_ID")
        .expect("Expected app id in the environment")
        .parse()
        .unwrap();

    let framework = Framework::new().command(test_cmd);

    let mut client = Client::builder(token)
        .event_handler(framework)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
