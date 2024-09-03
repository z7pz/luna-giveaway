use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};

use crate::middlewares::auth::{DiscordUser, Guilds};

use super::*;

#[axum::debug_handler]
async fn user(user: DiscordUser) -> impl IntoResponse {
    Json(user)
}

#[axum::debug_handler]
async fn guilds(guilds: Guilds, State(_): State<AppState>) -> impl IntoResponse {
    Json(guilds)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(user))
        .route("/guilds", get(guilds))
}
