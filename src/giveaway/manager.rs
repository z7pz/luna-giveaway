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
use dashmap::DashMap;
use crate::prelude::*;

use super::{
    giveaway::Giveaway,
    options::{GiveawayOptions, StartMessage},
    task::GiveawayTask,
};

pub struct GiveawayManager {
    pub tx: Sender<Arc<Mutex<Giveaway>>>,
    pub giveaways: DashMap<MessageId, Arc<Mutex<Giveaway>>>,
    pub tasks: DashMap<MessageId, GiveawayTask>,
}
impl GiveawayManager {
    pub async fn new(tx: Sender<Arc<Mutex<Giveaway>>>) -> Self {
        let giveaways = DashMap::new();
        let tasks = DashMap::new();

        Self {
            tx,
            giveaways,
            tasks,
        }
    }
    pub async fn create(&self, ctx: &Context<'_>, options: GiveawayOptions) -> Result<(), Error> {
        let tx = self.tx.clone();

        // sending giveaway start message
        let message = options
            .send_message(
                ctx.serenity_context().http.clone(),
                ctx.channel_id(),
                &vec![],
            )
            .await?;

        let giveaway = Arc::new(Mutex::new(Giveaway {
            message_id: message.id,
            options,
            is_ended: false,
            entries: vec![],
        }));

        // spawn a task to send giveaway to the tx channel when giveaway ends
        let g = giveaway.clone();
        let timer = {giveaway.lock().await}.options.timer;
        let giveaways = self.giveaways.clone();
        let task = tokio::spawn(async move {
            sleep(timer).await;
            {
                let g = g.lock().await;
                giveaways.remove(&g.message_id);
                println!("{}", giveaways.len())
            }
            if let Err(error) = tx.send(g).await {
                println!("Error: {:?}", error);
            };
        });
        self.giveaways
            .insert(message.id, giveaway.clone());
        self.tasks
            .insert(message.id, GiveawayTask { giveaway, task });
        Ok(())
    }
    pub fn end(&self) {}
    pub fn reroll(&self) {}
    pub fn hydrate(&self) {
        // get all giveaways from the database
        // for each giveaway, create a task
        // if the giveaway is not ended, sleep for the remaining time
        // if the giveaway is ended, send the giveaway to the tx channel
    }
}
