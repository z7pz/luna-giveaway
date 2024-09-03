use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};

use crate::prelude::*;

#[axum::debug_handler]
async fn commands(State(state): State<AppState>) -> impl IntoResponse {
    Json(state.data.commands)
}

#[axum::debug_handler]
async fn giveaways(State(state): State<AppState>) -> impl IntoResponse {
    Json(state.data.manager.entity.find_not_ended().await.unwrap())
}

pub fn routes() -> Router<AppState> {
    Router::new()
    .route("/commands", get(commands))
    .route("/giveaways", get(giveaways))
}
