use axum::{
    async_trait,
    extract::{FromRequestParts, Request, State},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use poise::serenity_prelude::{self, GuildInfo, Http};
use prisma_client::db::{
    self,
    oauth,
    user,
};
use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::structures::session::Session;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub banner: Option<String>,
    pub accent_color: Option<u32>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<u32>,
    pub premium_type: Option<u8>,
    pub public_flags: Option<u32>,
}
#[async_trait]
impl<S> FromRequestParts<S> for DiscordUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(parts.extensions.get::<Self>().unwrap().clone())
    }
}

async fn get_discord_user(access_token: &String) -> Result<DiscordUser, reqwest::Error> {
    Ok(reqwest::Client::new()
        .get("https://discord.com/api/v10/users/@me")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?
        .json::<DiscordUser>()
        .await?)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Guilds(pub Vec<GuildInfo>);

#[async_trait]
impl FromRequestParts<AppState> for Guilds {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let oauth = parts.extensions.get::<db::oauth::Data>().unwrap();
        Ok(get_discord_guilds(oauth.access_token.clone())
            .await
            .map(|guilds| {
                Self(
                    guilds
                        .iter()
                        .filter(|g| state.cache.guilds().contains(&g.id))
                        .cloned()
                        .collect::<_>(),
                )
            })
            .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()).into_response())?)
    }
}

pub async fn get_discord_guilds(
    access_token: String,
) -> Result<Vec<GuildInfo>, serenity_prelude::Error> {
    Http::new(format!("Bearer {}", access_token).as_str())
        .get_guilds(None, None)
        .await
}

async fn verify_user(
    session: Session,
    AppState { data: state, .. }: &AppState,
) -> Result<(DiscordUser, db::user::Data, db::oauth::Data), Response> {
    // TODO  caching
    let oauth = state
        .prisma
        .oauth()
        .find_first(vec![oauth::id::equals(session.id)])
        .exec()
        .await
        .ok()
        .and_then(|res| res)
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Unauthorized").into_response())?;

    let discord = get_discord_user(&oauth.access_token)
        .await
        .ok()
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Unauthorized").into_response())?;

    let profile = match state
        .prisma
        .user()
        .find_first(vec![user::id::equals(oauth.user_id)])
        .exec()
        .await
        .ok()
        .and_then(|f| f)
    {
        Some(profile) => profile,
        None => unreachable!(), // TODO check if actual not reachable
    };
    // send all back
    Ok((discord, profile, oauth))
}

pub async fn auth(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    let session = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| Session::decode(h.to_string()).ok())
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Unauthorized").into_response())?;
    let (user, profile, oauth) = verify_user(session, &state).await?;

    req.extensions_mut().insert(user);
    req.extensions_mut().insert(profile);
    req.extensions_mut().insert(oauth);

    Ok(next.run(req).await)
}
