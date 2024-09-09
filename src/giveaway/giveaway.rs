use chrono::Local;
use poise::serenity_prelude::{CacheHttp, Message, MessageId, UserId};
use prisma_client::db::giveaway;
use rand::seq::SliceRandom;
use serenity::EditMessage;

use super::options::{EndMessage, GiveawayOptions, StartMessage};
use crate::{entities::*, prelude::*};

pub struct Giveaway {
    entity: GiveawayEntity,
    pub message_id: MessageId,
    pub options: GiveawayOptions,
    pub entries: Vec<UserId>,
    pub is_ended: bool,
}
impl Giveaway {
    pub async fn from_data(giveaway: giveaway::Data) -> Self {
        let delta = giveaway.end_at.timestamp() - Local::now().fixed_offset().timestamp();
        Self {
            entity: GiveawayEntity::new(),
            message_id: MessageId::new(giveaway.message_id as u64),
            options: GiveawayOptions::from_data(giveaway).await,
            entries: vec![],
            is_ended: delta < 5,
        }
    }
}

impl Giveaway {
    pub fn new(message_id: MessageId, options: GiveawayOptions) -> Self {
        Self {
            entity: GiveawayEntity::new(),
            message_id,
            options,
            entries: vec![],
            is_ended: false,
        }
    }
    pub async fn save(&self) -> Result<giveaway::Data> {
        self.entity.create(self).await
    }
    pub async fn add_entry(
        &mut self,
        user_id: UserId,
        cache_http: impl CacheHttp,
    ) -> Result<Message, Error> {
        println!("Adding entry: {}", user_id);
        if self.is_ended {
            return Err("Giveaway has ended".into());
        }
        if self.entries.contains(&user_id) {
            return Err("You have already entered the giveaway".into());
        }

        let (gid, uid) = (self.message_id.clone(), user_id.clone());
        self.entries.push(user_id);
        tokio::spawn(async move {
            let user_entity = UserEntity::new(uid);
            user_entity
                .join_giveaway(&gid)
                .await
                .expect("Couldn't join the giveaway (DB)...");
        });
        self.update_message(
            cache_http,
            StartMessage::edit_message(&self.options, &self.entries),
        )
        .await
    }
    pub async fn update_message(
        &self,
        cache_http: impl CacheHttp,
        edit_message: EditMessage,
    ) -> Result<Message, Error> {
        Ok(self
            .options
            .channel_id
            .edit_message(cache_http, self.message_id, edit_message)
            .await?)
    }
    pub async fn end(&mut self, cache_http: impl CacheHttp) -> Result<Message, Error> {
        self.is_ended = true;
        let winners = self.get_winners();
        println!("{:?}", winners);
        // TODO send a new message with the winners
        let thread_winners = winners.clone();
        let thread_id = self.message_id.clone();
        tokio::spawn(async move {
            let entity = GiveawayEntity::new();
            entity
                .set_winners(
                    &thread_id,
                    thread_winners
                        .iter()
                        .map(|&c| c.clone().into())
                        .collect::<Vec<_>>(),
                )
                .await
                .expect("Couldn't set winners (DB)...");
        });
        self.entity.end(&self.message_id).await?;
        self.update_message(
            cache_http,
            EndMessage::edit_message(&self.options, &self.entries, &winners),
        )
        .await
    }
    pub fn get_winners(&self) -> Vec<UserId> {
        self.entries
            .choose_multiple(&mut rand::thread_rng(), self.options.winners_count as usize)
            .cloned()
            .collect::<Vec<UserId>>()
    }
}
