use serenity::prelude::*;

use crate::config::Config;

mod handler;
use self::handler::Handler;

pub struct Discord {
    client: Client,
}

impl Discord {
    pub async fn new() -> Self {
        let config = Config::load();

        let client = Client::builder(config.token)
            .event_handler(Handler::new())
            .application_id(config.application_id)
            .await
            .expect("Error creating client");

        Self { client }
    }

    pub async fn start(&mut self) {
        if let Err(why) = &mut self.client.start().await {
            println!("Client error: {:?}", why);
        }
    }
}
