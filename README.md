<p align="center">
    <img src="https://github.com/ImpossibleReality/ataraxy/blob/main/images/ataraxy_title.png?raw=true" alt="Ataraxy"/><br />
    <i>Discord slash commands framework for Serenity inspired by <code>Poise</code></i>
</p>

---

## Usage

Ataraxy is based off of the `#[command]` macro, which wraps a function and turns it into a usable command.

```rust
/// Says "Hello world"
#[command]
async fn say_hello(
    ctx: Context,
    #[option(channel_type = "text", description = "Text channel to say hello to")]
    channel: ChannelId,
) {
    channel
        .send_message(&ctx.http(), |m| m.content("Hello, world!"))
        .await;
    ctx.reply_ephemeral("Message sent successfully.").await;
}
```

You can then register commands and command groups to ataraxy's `Framework`.

```rust
let framework = Framework::new().command(test_cmd);
```

Ataraxy's `Framework` implements Serenity's `EventHandler` trait so that you can use it in the serenity `Client`

```rust
let mut client = Client::builder(token)
    .event_handler(framework)
    .application_id(application_id)
    .await
    .expect("Error creating client");

if let Err(why) = client.start().await {
    println!("Client error: {:?}", why);
}
```

It's as easy as that!




