
mod public;
mod user;
mod discord;
mod guild;

use axum::{
    extract::State, middleware::{self}, response::IntoResponse, routing::*, Router
};

use crate::{middlewares, prelude::*};


async fn root(State(state): State<AppState>) -> impl IntoResponse {
    state.data.manager.giveaways.len().to_string()
}

pub fn mount(app: Router<AppState>, state: AppState) -> Router<AppState> {
    app //
    .nest("/guild/:id", guild::routes())
    .layer(middleware::from_fn_with_state(state.clone(), middlewares::server::server))
    .nest("/user", user::routes())
    .layer(middleware::from_fn_with_state(state, middlewares::auth::auth))
    .nest("/public", public::routes())
    .nest("/discord", discord::routes())
    .route("/", get(root))
}

