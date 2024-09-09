use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde_json::json;
use serenity::MessageId;

use crate::{prelude::*, transformers::GiveawaysResponse, utils::merge};

#[axum::debug_handler]
async fn commands(State(state): State<AppState>) -> impl IntoResponse {
    Json(state.data.commands)
}

#[axum::debug_handler]
async fn giveaways(State(state): State<AppState>) -> impl IntoResponse {
    let data = GiveawaysResponse::from_db(
        state.data.manager.entity.find_not_ended().await.unwrap(),
        state.cache,
    );

    Json(json!(data))
}

#[axum::debug_handler]
async fn giveaway_details(
    State(state): State<AppState>,
    Path(id): Path<MessageId>,
) -> impl IntoResponse {
    Json(state.data.manager.entity.find_by_id(&id).await.unwrap())
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/commands", get(commands))
        .route("/giveaways", get(giveaways))
        .route("/giveaways/:id", get(giveaway_details))
}
