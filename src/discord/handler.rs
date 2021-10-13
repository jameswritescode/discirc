use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        interactions::{Interaction, InteractionResponseType},
    },
    prelude::*,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => "Pong!".to_string(),
                _ => "not implemented".to_string(),
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
