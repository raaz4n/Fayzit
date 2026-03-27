mod commands;
mod config;
use dotenv::dotenv;

use serenity::all::{Command, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "stats" => Some(commands::stats::run(&ctx, &command).await),
                _ => None,
            };

            if let Some(content) = content {
                let builder = CreateInteractionResponse::Message(content);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to the slash command: {why}");
                }
            }
        }
    }
    
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let global_command = 
            Command::create_global_command(&ctx.http, commands::stats::statistics()).await;

        if let Err(why) = global_command {
            println!("Failed to register command: {why:?}")
        }
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
    let mut client = Client::builder(&*config::BOTTOKEN, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client err: {why:?}");
    }
}
