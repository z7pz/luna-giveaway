use chrono::{DateTime, Local};
use poise::serenity_prelude::{
    ChannelId, CreateEmbed, CreateMessage, EditMessage, GuildId, Http, Message, UserId,
};
use serenity::{async_trait, CreateActionRow, CreateButton};
use std::{sync::Arc, time::Duration, vec};

use crate::prelude::*;

#[derive(Debug)]
pub struct GiveawayOptions {
    pub prize: String,
    pub winners: u32,
    pub timer: Duration,
    pub host: String,
    pub channel_id: ChannelId,
    pub guild_id: GuildId,
    pub starts_at: DateTime<Local>,
    pub ends_at: DateTime<Local>,
}

impl GiveawayOptions {
    pub fn new(ctx: &Context<'_>, prize: String, winners: u32, timer: Duration) -> Self {
        Self {
            prize,
            winners,
            timer,
            host: ctx.author().to_string(),
            channel_id: ctx.channel_id(),
            guild_id: ctx.guild_id().expect("Failed to get the guild id"), // WARN unwrap
            starts_at: Local::now(),
            ends_at: Local::now() + timer,
        }
    }
}

#[async_trait]
pub trait StartMessage {
    fn message_description(&self, entries: &Vec<UserId>) -> String;
    fn embed(&self, entries: &Vec<UserId>) -> CreateEmbed;
    fn create_message(&self, entries: &Vec<UserId>) -> CreateMessage;
    fn edit_message(&self, entries: &Vec<UserId>) -> EditMessage;
    async fn send_message(
        &self,
        http: Arc<Http>,
        channel_id: ChannelId,
        entries: &Vec<UserId>,
    ) -> Result<Message, Error>;
    fn buttons(&self) -> CreateActionRow;
}
#[async_trait]
pub trait EndMessage {
    fn message_description(&self, entries: &Vec<UserId>, winners: Vec<&UserId>) -> String;
    fn embed(&self, entries: &Vec<UserId>, winners: Vec<&UserId>) -> CreateEmbed;
    fn create_message(&self, entries: &Vec<UserId>, winners: Vec<&UserId>) -> CreateMessage;
    fn edit_message(&self, entries: &Vec<UserId>, winners: Vec<&UserId>) -> EditMessage;
    async fn send_message(
        &self,
        http: Arc<Http>,
        channel_id: ChannelId,
        entries: &Vec<UserId>,
        winners: Vec<&UserId>,
    ) -> Result<Message, Error>;
    fn buttons(&self) -> CreateActionRow;
}
#[async_trait]
impl StartMessage for GiveawayOptions {
    fn message_description(&self, entries: &Vec<UserId>) -> String {
        format!(
            "Prize: {}\nEntries: {}\nWinners: {}\nTime: <t:{3}:R> <t:{3}>",
            self.prize,
            entries.len(),
            self.winners,
            self.ends_at.timestamp(),
        )
    }
    fn embed(&self, entries: &Vec<UserId>) -> CreateEmbed {
        // TODO: get Start message embed config

        CreateEmbed::default()
            .title("Giveaway")
            .description(StartMessage::message_description(self, entries))
            .color((255, 0, 0))
    }
    fn create_message(&self, entries: &Vec<UserId>) -> CreateMessage {
        CreateMessage::new()
            .embed(StartMessage::embed(self, entries))
            .components(vec![StartMessage::buttons(self)])
    }
    fn buttons(&self) -> CreateActionRow {
        CreateActionRow::Buttons(vec![CreateButton::new("giveaway").label("Enter")])
    }
    fn edit_message(&self, entries: &Vec<UserId>) -> EditMessage {
        EditMessage::new().embed(StartMessage::embed(self, entries))
    }
    async fn send_message(
        &self,
        http: Arc<Http>,
        channel_id: ChannelId,
        entries: &Vec<UserId>,
    ) -> Result<Message, Error> {
        Ok(channel_id
            .send_message(http, StartMessage::create_message(self, entries))
            .await?)
    }
}

#[async_trait]
impl EndMessage for GiveawayOptions {
    fn message_description(&self, entries: &Vec<UserId>, winners: Vec<&UserId>) -> String {
        format!(
            "Prize: {}\nEntries: {}\nWinners: {}\nTime: <t:{3}:R> <t:{3}>",
            self.prize,
            entries.len(),
            winners
                .iter()
                .map(|u| format!("<@{}>", u))
                .collect::<Vec<_>>()
                .join(", "),
            self.ends_at.timestamp(),
        )
    }
    fn embed(&self, entries: &Vec<UserId>, winners: Vec<&UserId>) -> CreateEmbed {
        CreateEmbed::default()
            .title("Giveaway")
            .description(EndMessage::message_description(self, entries, winners))
            .color(0x00ff00)
    }
    fn create_message(&self, entries: &Vec<UserId>, winners: Vec<&UserId>) -> CreateMessage {
        CreateMessage::new()
            .embed(EndMessage::embed(self, entries, winners))
            .components(vec![EndMessage::buttons(self)])
    }
    fn edit_message(&self, entries: &Vec<UserId>, winners: Vec<&UserId>) -> EditMessage {
        EditMessage::new()
            .embed(EndMessage::embed(self, entries, winners))
            .components(vec![EndMessage::buttons(self)])
    }
    async fn send_message(
        &self,
        http: Arc<Http>,
        channel_id: ChannelId,
        entries: &Vec<UserId>,
        winners: Vec<&UserId>,
    ) -> Result<Message, Error> {
        Ok(channel_id
            .send_message(http, EndMessage::create_message(self, entries, winners))
            .await?)
    }
    fn buttons(&self) -> CreateActionRow {
        CreateActionRow::Buttons(vec![
            CreateButton::new_link("https://google.com").label("Giveaway")
        ])
    }
}
