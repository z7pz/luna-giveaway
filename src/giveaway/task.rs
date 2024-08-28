use std::{sync::Arc, time::Duration};

use futures::lock::Mutex;
use poise::serenity_prelude::MessageId;
use tokio::{task::JoinHandle, time::sleep};

use super::{giveaway::Giveaway, manager::GiveawayManager};

#[derive(Debug)]

pub struct GiveawayTask {
    pub giveaway:  Arc<Mutex<Giveaway>>,
    pub task: JoinHandle<()>,
}

impl GiveawayTask {
    pub async fn create_task(manager: &GiveawayManager, message_id: MessageId, giveaway: Arc<Mutex<Giveaway>>) -> Self {
        let timer = giveaway.lock().await.options.timer;
        let tx = manager.tx.clone();
        let g = giveaway.clone();
        let giveaways = manager.giveaways.clone();
        let tasks = manager.tasks.clone();

         Self {
            giveaway: giveaway.clone(),
            task: tokio::spawn(async move {
                sleep(timer).await;
                if let Err(error) = tx.send(g).await {
                    println!("Error: {:?}", error);
                };
                giveaways.remove(&message_id);
                tasks.remove(&message_id);
            })
         }
    }
}