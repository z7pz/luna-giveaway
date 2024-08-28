#![allow(unused)]
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{entities::*, get_prisma, prelude::*};
use chrono::{DateTime, Duration as ChronoDuration, Utc};
use dashmap::DashMap;
use futures::{lock::Mutex, FutureExt};
use once_cell::sync::OnceCell;
use poise::serenity_prelude::{
    CacheHttp, ChannelId, CreateEmbed, CreateMessage, GuildId, Http, MessageId, UserId,
};
use serenity::{EditMessage, Message};
use tokio::{sync::mpsc::Sender, time::sleep};

use super::{
    giveaway::Giveaway,
    options::{GiveawayOptions, StartMessage},
    task::GiveawayTask,
};

pub struct GiveawayManager {
    entity: Arc<GiveawayEntity>,
    pub tx: Sender<Arc<Mutex<Giveaway>>>,
    pub giveaways: Arc<DashMap<MessageId, Arc<Mutex<Giveaway>>>>,
    pub tasks: Arc<DashMap<MessageId, GiveawayTask>>,
}
impl GiveawayManager {
    pub async fn new(tx: Sender<Arc<Mutex<Giveaway>>>) -> Self {
        let giveaways = Arc::new(DashMap::new());
        let tasks = Arc::new(DashMap::new());
        let entity = Arc::new(GiveawayEntity::new());
        Self {
            entity,
            tx,
            giveaways,
            tasks,
        }
    }
    pub async fn create(&self, ctx: &Context<'_>, options: GiveawayOptions) -> Result<(), Error> {
        let prisma = get_prisma();

        // sending giveaway start message
        let message = options
            .send_message(
                ctx.serenity_context().http.clone(),
                ctx.channel_id(),
                &vec![],
            )
            .await?;

        let giveaway = Giveaway::new(message.id, options);

        if let Err(err) = giveaway.save().await {
            println!("Creating giveaway entity error: {err:?}");
            return Err(err);
        };

        let giveaway = Arc::new(Mutex::new(giveaway));

        // spawn a task to send giveaway to the tx channel when giveaway ends

        let task = self.create_task(message.id, giveaway.clone()).await;

        self.giveaways.insert(message.id, giveaway.clone());
        self.tasks.insert(message.id, task);
        Ok(())
    }
    async fn create_task(
        &self,
        message_id: MessageId,
        giveaway: Arc<Mutex<Giveaway>>,
    ) -> GiveawayTask {
        GiveawayTask::create_task(self, message_id, giveaway.clone()).await
    }
    pub fn end(&self) {}
    pub fn reroll(&self) {}
    pub async fn hydrate(&self) {
        // get all giveaways from the database
        // for each giveaway, create a task
        // if the giveaway is not ended, sleep for the remaining time
        // if the giveaway is ended, send the giveaway to the tx channel
    }
}
