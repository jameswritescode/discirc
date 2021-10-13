pub mod config;

pub mod discord;
use discord::bot::Bot;

#[tokio::main]
async fn main() {
    Bot::start().await;
}
