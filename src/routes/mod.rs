
use axum::{
    extract::State, middleware::{self}, response::IntoResponse, routing::*, Router
};

use crate::prelude::*;


async fn root(State(data): State<Data>) -> impl IntoResponse {
    data.manager.giveaways.len().to_string()
}

pub fn mount(app: Router<Data>, state: Data) -> Router<Data> {
    // im hating this
    app.route("/", get(root))
        // .nest("/auth", auth::routes())
        // .nest(
        //     "/party",
        //     party::routes().layer(middleware::from_fn_with_state(state, middlewares::auth)),
        // )
}

