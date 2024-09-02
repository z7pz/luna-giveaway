use chrono::{DateTime, FixedOffset, Local};
use poise::serenity_prelude::{
    ChannelId, CreateEmbed, CreateMessage, EditMessage, GuildId, Http, Message, UserId,
};
use prisma_client::db::{self, giveaway, guild};
use serenity::{async_trait, CreateActionRow, CreateButton, ReactionType};
use std::mem;
use std::str::FromStr;
use std::{sync::Arc, time::Duration, vec};

use crate::commands::EntryType;
use crate::{prelude::*, GuildEntity};

pub struct GiveawayOptions {
    pub entry_type: EntryType,
    pub guild: guild::Data,
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
    pub async fn from_ctx(
        ctx: &Context<'_>,
        prize: String,
        winners: u32,
        timer: Duration,
        entry_type: Option<EntryType>,
    ) -> Self {
        Self::new(
            ctx.author().to_string(),
            ctx.channel_id(),
            ctx.guild_id().expect("Guild not found!"),
            prize,
            winners,
            timer,
            entry_type,
        )
        .await
    }
    pub async fn from_data(giveaway: giveaway::Data) -> Self {
        let delta = giveaway.end_at.timestamp() - Local::now().fixed_offset().timestamp();
        Self::new(
            giveaway.host,
            ChannelId::new(giveaway.channel_id as u64),
            GuildId::new(giveaway.guild_id as u64),
            giveaway.prize,
            giveaway.winners as u32,
            Duration::new(if delta < 5 { 0 } else { delta as u64 }, 0),
            // TODO add entry type to schma
            None,
        )
        .await
    }
    pub async fn new(
        host: String,
        channel_id: ChannelId,
        guild_id: GuildId,
        prize: String,
        winners: u32,
        timer: Duration,
        entry_type: Option<EntryType>,
    ) -> Self {
        let guild_entity = GuildEntity::new(guild_id);
        let guild = guild_entity
            .find_or_create()
            .await
            .expect("Couldnt create guild");
        let default_entry_type: EntryType = unsafe { mem::transmute(guild.entry_type) };

        Self {
            guild,
            prize,
            winners,
            timer,
            host,
            channel_id,
            guild_id, 
            starts_at: Local::now(),
            ends_at: Local::now() + timer,
            entry_type: entry_type.unwrap_or(default_entry_type)
        }
    }
    fn parse_winners(&self, winners: &Vec<&UserId>) -> String {
        winners
            .iter()
            .map(|u| format!("<@{}>", u))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[async_trait]
pub trait StartMessage {
    fn message_title(&self, entries: &Vec<UserId>) -> String;
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
    fn message_title(&self, entries: &Vec<UserId>, winners: &Vec<&UserId>) -> String;
    fn message_description(&self, entries: &Vec<UserId>, winners: &Vec<&UserId>) -> String;
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
        parse_start_options(
            self,
            entries,
            &self
                .guild
                .end_embed_settings()
                .expect("End embed settings not found")
                .description,
        )
    }
    fn message_title(&self, entries: &Vec<UserId>) -> String {
        parse_start_options(
            self,
            entries,
            &self
                .guild
                .end_embed_settings()
                .expect("End embed settings not found")
                .title,
        )
    }
    fn embed(&self, entries: &Vec<UserId>) -> CreateEmbed {
        CreateEmbed::default()
            .title(StartMessage::message_title(self, entries))
            .description(StartMessage::message_description(self, entries))
            // TODO set color from guild settings
            .color((255, 0, 0))
    }
    fn create_message(&self, entries: &Vec<UserId>) -> CreateMessage {
        let mut message = CreateMessage::new()
            .embed(StartMessage::embed(self, entries));
        if self.entry_type == EntryType::Button {
            message = message.components(vec![StartMessage::buttons(self)]);
        }
        message
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
        let message = channel_id
        .send_message(http.clone(), StartMessage::create_message(self, entries))
        .await?;
        if self.entry_type == EntryType::Reaction {
            message.react(http, ReactionType::from_str(&self.guild.reaction)?).await?;
        }
        Ok(message)
    }
}

#[async_trait]
impl EndMessage for GiveawayOptions {
    fn message_description(&self, entries: &Vec<UserId>, winners: &Vec<&UserId>) -> String {
        parse_end_options(
            self,
            entries,
            winners,
            &self
                .guild
                .end_embed_settings()
                .expect("End embed settings not found")
                .description,
        )
    }
    fn message_title(&self, entries: &Vec<UserId>, winners: &Vec<&UserId>) -> String {
        parse_end_options(
            self,
            entries,
            winners,
            &self
                .guild
                .end_embed_settings()
                .expect("End embed settings not found")
                .title,
        )
    }
    fn embed(&self, entries: &Vec<UserId>, winners: Vec<&UserId>) -> CreateEmbed {
        // self.guild.end_embed_settings().expect("End embed settings not found").color
        CreateEmbed::default()
            .title(EndMessage::message_title(self, entries, &winners))
            .description(EndMessage::message_description(self, entries, &winners))
            // TODO set color from guild settings
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

fn parse_end_options(
    options: &GiveawayOptions,
    entries: &Vec<UserId>,
    winners: &Vec<&UserId>,
    text: &str,
) -> String {
    text.replace("{{winners}}", &options.parse_winners(&winners))
        .replace("{{prize}}", &options.prize)
        .replace("{{entries_count}}", &entries.len().to_string())
        .replace(
            "{{timer}}",
            &format!("<t:{}:R>", options.ends_at.timestamp()),
        )
        .replace(
            "{{ends_at}}",
            &format!("<t:{}>", options.ends_at.timestamp()),
        )
        .replace(
            "{{ends_at}}",
            &format!("<t:{}>", options.ends_at.timestamp()),
        )
}

fn parse_start_options(options: &GiveawayOptions, entries: &Vec<UserId>, text: &str) -> String {
    text.replace("{{prize}}", &options.prize)
        .replace("{{entries_count}}", &entries.len().to_string())
        .replace(
            "{{timer}}",
            &format!("<t:{}:R>", options.ends_at.timestamp()),
        )
        .replace(
            "{{ends_at}}",
            &format!("<t:{}>", options.ends_at.timestamp()),
        )
        .replace(
            "{{ends_at}}",
            &format!("<t:{}>", options.ends_at.timestamp()),
        )
}
