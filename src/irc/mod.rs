use tokio::sync::mpsc;

use crate::message::Message;

pub struct IRC {
    pub receiver: mpsc::Receiver<Message>,
    pub sender: mpsc::Sender<Message>,
}

impl IRC {
    pub fn start(&self) {}

    // async fn create_connection(&mut self, cache: Arc<Cache>) {
    //     // TODO: look at channels and build a tree based off category -> channels
    //     // TODO: Create or grab webhooks for each channel so we can use webhooks as users
    //     for (_, category) in cache.categories().await.iter() {
    //         let res: Vec<&str> = category.name.split("/").collect();

    //         let config = Config {
    //             nickname: Some("discirc".to_owned()),
    //             port: Some(res[1].parse::<u16>().unwrap()),
    //             server: Some(res[0].to_owned()),
    //             ..Config::default()
    //         };

    //         let client = Client::from_config(config).await.unwrap();

    //         self.connections.push(client);
    //     }
    // }
}
