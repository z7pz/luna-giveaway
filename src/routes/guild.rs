use std::{f64::consts::E, ops::Deref, sync::Arc};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use middlewares::server::Guild;
use prisma_client::db::{giveaway::{self, Actions}, EntryType};
use prisma_client_rust::{prisma_models::OrderBy, OrderByQuery};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serenity::{Cache, GuildId, MessageId, RoleId};



use unicode_segmentation::UnicodeSegmentation;

fn is_emoji(s: &str) -> bool {
    s.graphemes(true).all(|g| {
        g.chars().all(|c| c.is_emoji())
    })
}

trait EmojiCheck {
    fn is_emoji(&self) -> bool;
}

impl EmojiCheck for char {
    fn is_emoji(&self) -> bool {
        // Emoji ranges based on Unicode standard
        match *self as u32 {
            0x1F600..=0x1F64F | // Emoticons
            0x1F300..=0x1F5FF | // Misc Symbols and Pictographs
            0x1F680..=0x1F6FF | // Transport and Map Symbols
            0x1F700..=0x1F77F | // Alchemical Symbols
            0x1F780..=0x1F7FF | // Geometric Shapes Extended
            0x1F800..=0x1F8FF | // Supplemental Arrows-C
            0x1F900..=0x1F9FF | // Supplemental Symbols and Pictographs
            0x1FA00..=0x1FA6F | // Chess Symbols
            0x1FA70..=0x1FAFF | // Symbols and Pictographs Extended-A
            0x2600..=0x26FF |   // Misc symbols
            0x2700..=0x27BF |   // Dingbats
            0xFE0F..=0xFE0F => true, // Variation Selector-16
            _ => false,
        }
    }
}


use crate::{
    entities,
    transformers::{GiveawaysResponse, GuildWithGiveaways, UpdateSettings},
    GuildEntity,
};

use super::*;

#[axum::debug_handler]
async fn overview(State(state): State<AppState>, Guild(guild): Guild) -> impl IntoResponse {
    let entity = GuildEntity::new(&guild.id);
    let g = entity
        .find_one_with_giveaways()
        .await
        .unwrap()
        .expect("Failed to find guild");
    Json(json!(GuildWithGiveaways::from_db(g, state.cache)))
}

#[derive(Serialize, Deserialize)]
struct DisabledCommands(Vec<String>);

impl Deref for DisabledCommands {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[axum::debug_handler]
async fn update_commands(
    State(state): State<AppState>,
    Guild(guild): Guild,
    Json(disabled_commands): Json<DisabledCommands>,
) -> Result<impl IntoResponse> {
    if !disabled_commands
        .iter()
        .all(|c| state.data.commands.iter().find(|cmd| &cmd.name == c).is_some())
    {
        return Err(Error::CommandNotFound);
    }

    let entity = GuildEntity::new(&guild.id);
    entity.update_commands(disabled_commands.clone()).await?;

    Ok(Json(guild))
}


#[axum::debug_handler]
async fn roles(State(state): State<AppState>, Guild(guild): Guild) -> impl IntoResponse {
    Json(
        state
            .cache
            .guild(guild.id)
            .expect("Failed to find guild")
            .roles
            .clone(),
    )
}


fn validate_settings(id: GuildId, update_settings: UpdateSettings, cache: Arc<Cache>) -> Result<()> {
    if update_settings.creator_roles.is_empty() {
        return Err("No creator roles".into());
    }

    if update_settings.disabled_commands.is_empty() {
        return Err("No disabled commands".into());
    }

    if update_settings.reaction.is_empty() {
        return Err("No reaction".into());
    }

    if update_settings.start_embed_settings.color.is_empty() {
        return Err("No start embed color".into());
    }

    if update_settings.start_embed_settings.title.is_empty() {
        return Err("No start embed title".into());
    }

    if update_settings.start_embed_settings.description.is_empty() {
        return Err("No start embed description".into());
    }

    if update_settings.end_embed_settings.color.is_empty() {
        return Err("No end embed color".into());
    }

    if update_settings.end_embed_settings.title.is_empty() {
        return Err("No end embed title".into());
    }

    if update_settings.end_embed_settings.description.is_empty() {
        return Err("No end embed description".into());
    }

    if !is_emoji(&update_settings.reaction) {
        return Err("Invalid reaction".into());
    }

    // cache.guild(id).map(|g| g.emojis.iter().find(|c| c.1.))

    Ok(())
}

#[axum::debug_handler]
async fn update_settings(State(state): State<AppState>, Guild(guild): Guild, Json(update_settings): Json<UpdateSettings>) -> Result<impl IntoResponse> {
    validate_settings(guild.id, update_settings.clone(), state.cache.clone())?;
    let entity = GuildEntity::new(&guild.id);
    entity.update(update_settings.clone()).await?;
    Ok(())
}

#[axum::debug_handler]
async fn settings(Guild(guild): Guild) -> impl IntoResponse {
    let entity = GuildEntity::new(&guild.id);
    Json(
        entity
            .find_or_create()
            .await
            .expect("Failed to find or create guild"),
    )
}
#[axum::debug_handler]
async fn giveaways(State(state): State<AppState>, Guild(guild): Guild) -> impl IntoResponse {
    Json(GiveawaysResponse::from_db(
        state
            .data
            .prisma
            .giveaway()
            .find_many(vec![giveaway::guild_id::equals(guild.id.into())])
            .order_by(giveaway::is_ended::order(
                prisma_client_rust::Direction::Asc,
            ))
            .exec()
            .await
            .expect("Failed to find giveaways"),
        state.cache,
    ))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/roles", get(roles))
        .route("/overview", get(overview))
        .route("/commands", post(update_commands))
        .route("/settings", get(settings))
        .route("/settings", post(update_settings))
        .route("/giveaways", get(giveaways))
}
