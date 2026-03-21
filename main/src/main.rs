mod commands;
mod config;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // placeholder function for checking bot status
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name)
    }
}

#[tokio::main]
async fn main() {
    // load .env, ignore errors
    dotenv().ok();

    // setting gateway intents, decides what events bot will be notified of
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // client setup
    let mut client = Client::builder(&*config::BOTTOKEN, intents).event_handler(Handler).await.expect("Error creating client");
    if let Err(why) = client.start().await {
        println!("Client err: {why:?}");
    }
}
