use std::sync::Arc;

use tokio::sync::mpsc;

use serenity::{
    async_trait,
    cache::Cache,
    model::{
        channel::Message as DiscordMessage,
        gateway::Ready,
        id::GuildId,
        interactions::{
            application_command::ApplicationCommandInteraction, Interaction,
            InteractionResponseType,
        },
    },
    prelude::*,
};

use crate::irc::IRC;
use crate::message::Message;

static COMMANDS: [(&str, &str); 6] = [
    ("ping", "Pong!"),
    ("join", "/join #channel"),
    ("part", "/part #channel"),
    ("whois", "/whois nick"),
    ("server", "Manage IRC Connections"),
    ("names", "Show users in IRC channel"),
];

pub struct Handler {
    receiver: Option<mpsc::Receiver<DiscordMessage>>,
    sender: Option<mpsc::Sender<DiscordMessage>>,
}

impl Handler {
    pub fn new() -> Self {
        Self {
            receiver: None,
            sender: None,
        }
    }

    fn discord_to_irc_command(&self, command: &ApplicationCommandInteraction) -> String {
        // TODO: Find the correct server to issue command to
        return command.data.name.as_str().to_string();
    }

    async fn irc_start(&self, cache: Arc<Cache>) {
        let (sender, mut receiver) = mpsc::channel::<DiscordMessage>(16);
        self.sender = Some(sender);

        // let (_irc_sender, irc_receiver) = mpsc::channel::<Message>(16);

        // let irc = IRC {
        //     receiver: irc_receiver,
        //     sender: discord_sender,
        // };

        // irc.start();

        tokio::spawn(async move {
            loop {
                let message = receiver.recv().await.unwrap();

                println!("got message {}", message.content);
            }
        });
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        println!("Cache build successfully");

        for guild in guilds.iter() {
            for (name, desc) in COMMANDS.iter() {
                guild
                    .create_application_command(&ctx.http, |c| c.name(name).description(desc))
                    .await
                    .unwrap();
            }
        }

        self.irc_start(ctx.cache).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => "Pong!".to_string(),
                _ => self.discord_to_irc_command(&command),
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

    async fn message(&self, _: Context, message: DiscordMessage) {
        match self.sender {
            Some(sender) => sender.send(message).await.unwrap(),
            None => println!("Discord message received but no sender"),
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected to Discord", ready.user.name);
    }
}
