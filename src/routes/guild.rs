use std::{f64::consts::E, ops::Deref};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use middlewares::server::Guild;
use prisma_client::db::giveaway::{self, Actions};
use prisma_client_rust::{prisma_models::OrderBy, OrderByQuery};
use serde::{Deserialize, Serialize};
use serenity::MessageId;

use crate::{entities, GuildEntity};

use super::*;

#[axum::debug_handler]
async fn overview(Guild(guild): Guild) -> impl IntoResponse {
    let entity = GuildEntity::new(&guild.id);
    Json(entity.find_one_with_giveaways().await.expect("Failed to find guild"))
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
        .all(|c| state.data.commands.contains(c))
    {
        return Err(Error::CommandNotFound);
    }

    let entity = GuildEntity::new(&guild.id);
    entity.update_commands(disabled_commands.clone()).await?;

    Ok(Json(guild))
}

struct UpdateSettings {
    prefix: String,
}

#[axum::debug_handler]
async fn update_settings(Guild(guild): Guild) -> impl IntoResponse {
    Json(guild)
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
    Json(
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
    )
}


pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/overview", get(overview))
        .route("/commands", post(update_commands))
        .route("/settings", get(settings))
        .route("/settings", post(update_settings))
        .route("/giveaways", get(giveaways))
}
