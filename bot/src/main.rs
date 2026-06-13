use std::env;

use poise::PrefixFrameworkOptions;
use serenity::prelude::*;

mod commands;

type Error = Box<dyn std::error::Error + Send + Sync>;
type ContextPoise<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    vec_miam: Mutex<Vec<String>>,
}



#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let options = poise::FrameworkOptions{
        commands:vec![commands::help(),commands::rollmanger(),commands::addmanger(),commands::removemanger(),commands::seemanger(),],
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        }, 
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        }, 
        prefix_options: PrefixFrameworkOptions{ prefix: Some("!".into()), ..Default::default()},
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    vec_miam: Mutex::new(Vec::new()),
                })
            })
        })
        .options(options)
        .build();


    let mut client =
        Client::builder(&token, intents).framework(framework).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}