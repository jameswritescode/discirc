use std::fs::File;
use std::io::prelude::*;

use serde::Deserialize;

use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        interactions::{Interaction, InteractionResponseType},
    },
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => "Hey, I'm alive!".to_string(),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        for gs in ready.guilds.iter() {
            let guild = gs.id();

            let _ = guild
                .create_application_command(&ctx.http, |command| {
                    command.name("ping").description("A ping command")
                })
                .await;
        }
    }
}

#[derive(Deserialize)]
struct Config {
    application_id: u64,
    token: String,
}

#[tokio::main]
async fn main() {
    let mut file = File::open("discirc.toml").expect("Config Error");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Config Read Error");

    let config: Config = toml::from_str(&contents).unwrap();

    let mut client = Client::builder(config.token)
        .event_handler(Handler)
        .application_id(config.application_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
