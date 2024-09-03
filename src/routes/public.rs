
use axum::{extract::{Path, State}, response::IntoResponse, routing::get, Json, Router};
use serenity::MessageId;

use crate::prelude::*;

#[axum::debug_handler]
async fn commands(State(state): State<AppState>) -> impl IntoResponse {
    Json(state.data.commands)
}

#[axum::debug_handler]
async fn giveaways(State(state): State<AppState>) -> impl IntoResponse {
    Json(state.data.manager.entity.find_not_ended().await.unwrap())
}

#[axum::debug_handler]
async fn giveaway_details(State(state): State<AppState>, Path(id): Path<MessageId>) -> impl IntoResponse {
    Json(state.data.manager.entity.find_by_id(&id).await.unwrap())
}

pub fn routes() -> Router<AppState> {
    Router::new()
    .route("/commands", get(commands))
    .route("/giveaways", get(giveaways))
    .route("/giveaways/:id", get(giveaway_details))
}
