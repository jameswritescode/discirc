use serenity::prelude::Client;

use super::super::config::Config;
use super::handler::Handler;

pub struct Bot;

impl Bot {
    pub async fn start() {
        let config = Config::load();

        let mut client = Client::builder(config.token)
            .event_handler(Handler)
            .application_id(config.application_id)
            .await
            .expect("Error creating client");

        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
    }
}
