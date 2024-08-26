use poise::serenity_prelude::{CacheHttp, Message, MessageId, UserId};


use super::options::GiveawayOptions;
use crate::prelude::*;

#[derive(Debug)]
pub struct Giveaway {
    pub message_id: MessageId,
    pub args: GiveawayOptions,
    pub entries: Vec<UserId>,
    pub is_ended: bool,
}


impl Giveaway {
    pub async fn add_entry(&mut self, user_id: UserId, cache_http: impl CacheHttp) -> Result<Message, Error> {
        self.entries.push(user_id);
        Ok(self.update_message(cache_http).await?)
    }
    pub async fn update_message(&self, cache_http: impl CacheHttp) -> Result<Message, Error> {
        Ok(self.args.channel_id.edit_message(cache_http, self.message_id, self.args.edit_message(&self.entries)).await?)
    } 
}
