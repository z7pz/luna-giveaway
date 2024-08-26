use std::{sync::Arc, time::{Duration, SystemTime, UNIX_EPOCH}};

use poise::serenity_prelude::{ChannelId, CreateEmbed, CreateMessage, EditMessage, GuildId, Http, Message, UserId};

use crate::prelude::*;

#[derive(Debug)]
pub struct GiveawayOptions {
    pub prize: String,
    pub winners: u32,
    pub timer: Duration,
    pub host: String,
    pub channel_id: ChannelId,
    pub guild_id: GuildId,
    pub starts_at: Duration,
    pub ends_at: Duration,
}
impl GiveawayOptions {
    pub fn message_description(&self, entries: Vec<UserId>) -> String {
        format!(
            "Prize: {}\nEntries: {}\nWinners: {}\nTime: <t:{3}:R> <t:{3}>",
            self.prize,
            entries.len(),
            self.winners,
            self.ends_at.as_secs(),
        )
    }
    pub fn embed(&self, entries: Vec<UserId>) -> CreateEmbed {
        let embed = CreateEmbed::default()
            .title("Giveaway")
            .description(self.message_description(entries))
            .color(0x00ff00);

        embed
    }
    pub fn create_message(&self, entries: Vec<UserId>) -> CreateMessage {
        CreateMessage::new().embed(self.embed(entries))
    }
    pub fn edit_message(&self, entries: Vec<UserId>) -> EditMessage {
        EditMessage::new().embed(self.embed(entries))
    }
    pub async fn send_message(
        &self,
        http: Arc<Http>,
        channel_id: ChannelId,
        entries: Vec<UserId>,
    ) -> Result<Message, Error> {
        Ok(channel_id
            .send_message(http, self.create_message(entries))
            .await?)
    }
}



impl GiveawayOptions {
    pub fn new(ctx: &Context<'_>, prize: String, winners: u32, timer: Duration) -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        Self {
            prize,
            winners,
            timer,
            host: ctx.author().to_string(),
            channel_id: ctx.channel_id().into(),
            guild_id: ctx.guild_id().expect("Failed to get the guild id").into(), // WARN unwrap
            starts_at: since_the_epoch,
            ends_at: since_the_epoch + timer,
        }
    }
}