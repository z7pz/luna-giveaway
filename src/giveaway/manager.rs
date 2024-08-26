#![allow(unused)]
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use futures::{lock::Mutex, FutureExt};
use once_cell::sync::OnceCell;
use poise::serenity_prelude::{
    CacheHttp, ChannelId, CreateEmbed, CreateMessage, GuildId, Http, MessageId, UserId,
};
use serenity::{EditMessage, Message};
use tokio::{sync::mpsc::Sender, time::sleep};

use crate::prelude::*;

use super::{giveaway::Giveaway, options::GiveawayOptions, task::GiveawayTask};

pub struct GiveawayManager {
    pub tx: Sender<Arc<Giveaway>>,
    pub giveaways: Arc<Mutex<HashMap<MessageId, Giveaway>>>,
    pub tasks: Arc<Mutex<HashMap<MessageId, GiveawayTask>>>,
}
impl GiveawayManager {
    pub async fn new(tx: Sender<Arc<Giveaway>>) -> Self {
        let giveaways = Arc::new(Mutex::new(HashMap::new()));
        let tasks = Arc::new(Mutex::new(HashMap::new()));

        Self {
            tx,
            giveaways,
            tasks,
        }
    }
    pub async fn create(&self, ctx: &Context<'_>, args: GiveawayOptions) -> Result<(), Error> {
        let tx = self.tx.clone();

        // sending giveaway start message
        let message = args
            .send_message(
                ctx.serenity_context().http.clone(),
                ctx.channel_id(),
                vec![],
            )
            .await?;

        let giveaway = Arc::new(Giveaway {
            message_id: message.id,
            args,
            is_ended: false,
            entries: vec![],
        });

        // spawn a task to send giveaway to the tx channel when giveaway ends
        let g = giveaway.clone();
        let giveaways = self.giveaways.clone();
        let task = tokio::spawn(async move {
            // sleep(g.args.timer).await;
            {
                let mut giveaways = giveaways.lock().await;
                giveaways.remove(&g.message_id);
                println!("{}", giveaways.len())
            }
            if let Err(error) = tx.send(g).await {
                println!("Error: {:?}", error);
            };
        });

        self.tasks
            .lock()
            .await
            .insert(message.id, GiveawayTask { giveaway, task });
        Ok(())
    }
    pub fn end(&self) {}
    pub fn reroll(&self) {}
}
