use std::env;
use std::sync::{LazyLock};

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

static  VEC_MIAM: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!manger" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Hello").await {
                println!("Error sending message: {why:?}");
            }
        }

        if msg.content == "!addmanger" {
            let list_msg: Vec<&str>= msg.content.split(' ').collect();
            if list_msg.len() < 2{
                if let Err(why) = msg.channel_id.say(&ctx.http, "Need at least 2 args").await {
                    println!("Error sending message: {why:?}");
                }
                return ;
            }
            let foodname = list_msg.as_slice()[1..list_msg.len()].join("' '");
            let mut guard = VEC_MIAM.lock().await;
            guard.push(foodname);
            drop(guard);
        }

        if msg.content == "!removemanger"{
            let list_msg: Vec<&str>= msg.content.split(' ').collect();
            if list_msg.len() < 2{
                if let Err(why) = msg.channel_id.say(&ctx.http, "Need at least 2 args").await {
                    println!("Error sending message: {why:?}");
                }
                return ;
            }
            let foodname = list_msg.as_slice()[1..list_msg.len()].join("' '");
            let mut guard = VEC_MIAM.lock().await;
            guard.retain(|name| !name.eq(&foodname));
            drop(guard);
        }

        if msg.content == "!seemanger"{
            let guard = VEC_MIAM.lock().await;
            if let Err(why) = msg.channel_id.say(&ctx.http, format!("{:?}",guard)).await {
                    println!("Error sending message: {why:?}");
            }
        }
    }
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

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}