use dotenv::dotenv;
use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // handler for 'message', to be called when a new message is received
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "Hello" {
            // if the message fails due to anything
            if let Err(why) = msg.channel_id.say(&ctx.http, "World!").await {
                println!("Error sending msg: {why:?}");
            }
        }
    }

    // once bot is connected, give the status update
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // load .env, ignor errors
    dotenv().ok();
    // client config with .env bot token
    let token = env::var("TOKEN").expect("Need a token for the Discord bot");

    // setting gateway intents, decides what events bot will be notified of
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // new instance of the Client which will tie in with the discord bot token
    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // listen to events
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
