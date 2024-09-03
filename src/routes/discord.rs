use crate::{
    config::{DISCORD_ID, DISCORD_SECRET, FRONTEND_URI, GRANT_TYPE, REDIRECT_URI, SCOPES},
    structures::session::Session,
};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::*,
    Json, Router,
};
use prisma_client::db::{oauth, user};
use reqwest::{multipart, Client};

use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use url::Url;

use crate::prelude::*;

#[derive(Deserialize)]
struct RedirectQuery {
    code: String,
}
#[allow(unused)]
#[derive(Deserialize, Debug)]
struct TokenResponse {
    token_type: String,
    access_token: String,
    expires_in: u32,
    refresh_token: String,
    scope: String,
}

#[serde_as]
#[derive(Deserialize)]
struct UserResponse {
    #[serde_as(as = "DisplayFromStr")]
    id: i64,
}

#[axum::debug_handler]
async fn redirect(
    State(state): State<AppState>,
    Query(payload): Query<RedirectQuery>,
) -> Result<impl IntoResponse> {
    let client = Client::new();
    let form = multipart::Form::new()
        .text("client_id", &*DISCORD_ID)
        .text("client_secret", &*DISCORD_SECRET)
        .text("grant_type", &*GRANT_TYPE)
        .text("code", payload.code)
        .text("redirect_uri", &*REDIRECT_URI);

    let token_req = client
        .post("https://discord.com/api/v10/oauth2/token")
        .multipart(form)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;

    if token_req.status().as_u16() != 200 {
        return Ok((StatusCode::BAD_REQUEST, token_req.text().await?).into_response());
    }

    let token_json = token_req.json::<TokenResponse>().await?;

    if !validate_required_scopes(&token_json.scope) {
        return Ok((StatusCode::BAD_REQUEST, StatusCode::BAD_REQUEST.to_string()).into_response());
    }

    let user_req = client
        .get("https://discord.com/api/v10/users/@me")
        .header(
            "Authorization",
            format!("Bearer {}", token_json.access_token),
        )
        .send()
        .await?;

    let user_json = user_req.json::<UserResponse>().await?;

    let token: String = if let Some(user) = state
        .data
        .prisma
        .user()
        .find_first(vec![user::id::equals(user_json.id.clone())])
        .exec()
        .await?
    {
        let oauth = state
            .data
            .prisma
            .oauth()
            .upsert(
                oauth::user_id::equals(user.id),
                (
                    token_json.access_token.clone(),
                    token_json.refresh_token.clone(),
                    user::UniqueWhereParam::IdEquals(user_json.id.clone()),
                    vec![],
                ),
                vec![
                    oauth::access_token::set(token_json.access_token),
                    oauth::refresh_token::set(token_json.refresh_token),
                ],
            )
            .exec()
            .await?;
        Session::new(oauth.id).gen_token().unwrap()
    } else {
        let user = state
            .data
            .prisma
            .user()
            .create(user_json.id, vec![])
            .exec()
            .await?;
        let oauth = state
            .data
            .prisma
            .oauth()
            .create(
                token_json.access_token,
                token_json.refresh_token,
                user::UniqueWhereParam::IdEquals(user.id),
                vec![],
            )
            .exec()
            .await?;
        Session::new(oauth.id).gen_token().unwrap()
    };

    let mut url = Url::parse(&*FRONTEND_URI).expect("Invalid base URL");

    url.set_query(Some(&format!("token={}", token)));
    println!("{token:?}");
    let stoken = Session::decode(token.to_string());
    println!("{stoken:?}");

    Ok(Json(token).into_response())
    // Ok(Redirect::permanent(&url.to_string()).into_response())
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/redirect", get(redirect))
}
fn validate_required_scopes(str: &str) -> bool {
    (*SCOPES)
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .all(|&constant| str.contains(&constant))
}
