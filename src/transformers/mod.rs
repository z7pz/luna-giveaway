use std::sync::Arc;

use poise::serenity_prelude::{Cache, GuildId, RoleId, User, UserId};
use prisma_client::db::{giveaway, guild, user, EntryType};
use serde::{Deserialize, Serialize};

use crate::{Data, Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildResponse {
    pub id: GuildId,
    pub icon: String,
    pub name: String,
    pub member_count: u64,
}
impl GuildResponse {
    pub fn from_guild(guild: poise::serenity_prelude::Guild) -> Self {
        Self {
            id: guild.id,
            icon: guild.icon_url().unwrap_or_default(),
            name: guild.name.clone(),
            member_count: guild.member_count,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GiveawayResponse {
    #[serde(flatten)]
    pub data: giveaway::Data,
    // overriding the giveaway::Data fields
    pub entries: Vec<UserResponse>,
    pub winners: Vec<UserResponse>,
    pub guild: GuildResponse,
    
}

impl GiveawayResponse {
    pub fn from_db(data: giveaway::Data, cache: Arc<Cache>) -> Option<GiveawayResponse> {
        cache.guild(data.guild_id as u64).map(|g| GiveawayResponse {
            data: data.to_owned(),
            entries: data
                .clone()
                .entries
                .expect("Entries not found")
                .iter()
                .filter_map(|e| UserResponse::from_db(e.clone(), cache.clone()))
                .collect::<Vec<_>>(),
            winners: data
                .clone()
                .winners
                .expect("winners not found")
                .iter()
                .filter_map(|e| UserResponse::from_db(e.clone(), cache.clone()))
                .collect::<Vec<_>>(),
            guild: GuildResponse::from_guild(g.to_owned()),
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GiveawaysResponse(pub Vec<GiveawayResponse>);

impl GiveawaysResponse {
    pub fn from_db(giveaways: Vec<giveaway::Data>, cache: Arc<Cache>) -> Self {
        Self(
            giveaways
                .iter()
                .filter_map(|data| GiveawayResponse::from_db(data.clone(), cache.clone()))
                .collect::<Vec<_>>(),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse(pub User);

impl UserResponse {
    pub fn from_db(data: user::Data, cache: Arc<Cache>) -> Option<UserResponse> {
        cache
            .user(data.id as u64)
            .map(|u| u.to_owned())
            .map(|u| UserResponse(u))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildWithGiveaways {
    #[serde(flatten)]
    pub data: guild::Data,
    pub giveaways: Vec<GiveawayResponse>,
}
impl GuildWithGiveaways {
    pub fn from_db(data: guild::Data, cache: Arc<Cache>) -> GuildWithGiveaways {
        GuildWithGiveaways {
            data: data.to_owned(),
            giveaways: data
                .giveaways
                .unwrap()
                .iter()
                .filter_map(|g| GiveawayResponse::from_db(g.clone(), cache.clone()))
                .collect::<Vec<_>>(),
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommandResponse {
    pub name: String,
    pub category: String,
    pub description: String,
}
impl CommandResponse {
    pub fn from_command(poise::Command { name, category, description, .. }: &poise::Command<Data, Error>) -> Self {
        Self {
            name: name.to_string(),
            category: category.clone().unwrap_or("Giveaway".to_string()),
            description: description.clone().unwrap_or_default(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateEmbed {
    pub color: String,
    pub title: String,
    pub description: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateSettings {
    pub creator_roles: Vec<RoleId>,
    pub disabled_commands: Vec<String>,
    pub entry_type: EntryType,
    pub reaction: String,
    pub prefix: String,
    pub start_embed_settings: UpdateEmbed,
    pub end_embed_settings: UpdateEmbed,
}
