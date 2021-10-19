mod config;
mod irc;
mod message;

mod discord;
use discord::Discord;

#[tokio::main]
async fn main() {
    let mut bot = Discord::new().await;

    bot.start().await
}
