use std::sync::Arc;

use axum::{extract::{Path, State}, response::IntoResponse, routing::get, Json, Router};
use prisma_client::db;
use serde::{Deserialize, Serialize};
use serenity::{Cache, MessageId};

use crate::{middlewares::auth::{DiscordUser, Guilds}, transformers::GiveawayResponse};

use super::*;

#[derive(Serialize, Deserialize)]
struct UserProfile {
    giveaways: Vec<GiveawayResponse>,
}

impl UserProfile {
    pub fn from_db(user: db::user::Data, cache: Arc<Cache>) -> Self {
        Self {
            giveaways: user
                .giveaways
                .expect("Giveaways not found")
                .iter()
                .filter_map(|g| GiveawayResponse::from_db(g.clone(), cache.clone()))
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct UserResponse {
#[serde(flatten)]
    user: DiscordUser,
    profile: UserProfile,
}


#[axum::debug_handler]
async fn user(user: DiscordUser) -> impl IntoResponse {
    Json(user)
}

#[axum::debug_handler]
async fn join_giveaway(State(state): State<AppState>, user: DiscordUser, Path(id): Path<MessageId>) -> Result<impl IntoResponse> {
    match state.data.manager.giveaways.get(&id) {
        Some(giveaway) => {
            giveaway.lock().await.add_entry(user.id, state.http).await?;
            Ok(())
        },
        None => {
            Err("Giveaway not found".into())
        },
    }
}

#[axum::debug_handler]
async fn guilds(guilds: Guilds, State(_): State<AppState>) -> impl IntoResponse {
    Json(guilds)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(user))
        .route("/guilds", get(guilds))
        .route("/giveaways", get(guilds))
        .route("/giveaways/:id", post(join_giveaway))
}
