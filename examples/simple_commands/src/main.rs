use ataraxy::command;
use ataraxy::Context;
use ataraxy::Framework;
use dotenv::dotenv;
use serenity::Client;
use std::env;

/// Says "Hello world"
#[command]
async fn hello_world(ctx: Context, name: Option<String>) {
    ctx.reply(format!(
        "Hello there, {}",
        name.unwrap_or("Joe".to_string())
    ))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let application_id = env::var("APPLICATION_ID")
        .expect("Expected app id in the environment")
        .parse()
        .unwrap();

    let framework = Framework::new().command(hello_world);

    let mut client = Client::builder(token)
        .event_handler(framework)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
