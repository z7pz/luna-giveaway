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

#[derive(Clone)]
pub struct GiveawayManager {
    pub tx: Sender<Arc<Mutex<Giveaway>>>,
    pub giveaways: Arc<DashMap<MessageId, Arc<Mutex<Giveaway>>>>,
    pub tasks: Arc<DashMap<MessageId, GiveawayTask>>,
    entity: Arc<GiveawayEntity>,
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
    pub async fn hydrate(&self, http: Arc<Http>) {
        let giveaways = self
            .entity
            .find_not_ended()
            .await
            .expect("Error finding giveaways");
        println!("Hydrating giveaways: {}", giveaways.len());
        for giveaway in giveaways {
            let mut giveaway = Giveaway::from_data(giveaway).await;
            if giveaway.is_ended {
                let h = http.clone();
                tokio::spawn(async move {
                    let _ = giveaway.end(h).await;
                });
                continue;
            } else {
                let giveaway = Arc::new(Mutex::new(giveaway));
                let message_id = giveaway.lock().await.message_id;
                let task = self
                    .create_task(message_id.clone(), giveaway.clone())
                    .await;
                self.giveaways
                    .insert(message_id.clone(), giveaway.clone());
                self.tasks.insert(message_id, task);
            }
        }
    }
}
