extern crate reqwest;

use std::env;

use serenity::prelude::*;

mod handlers;
mod domain;

#[tokio::main]
async fn main() {
    let token: String = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents: GatewayIntents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Build client application
    let mut client: Client =
        Client::builder(&token, intents)
        .event_handler(handlers::WoodcordHandler)
            .await.expect("Err creating client");
    
    // Start Application
    if let Err(why) = client.start_shards(2).await {
        println!("Client error: {:?}", why);
    }
}