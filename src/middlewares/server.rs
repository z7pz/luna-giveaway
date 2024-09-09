use axum::{
    async_trait,
    extract::{FromRequestParts, Path, Request, State},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use poise::serenity_prelude::{self, GuildInfo, Http};
use prisma_client::db::{self, oauth, user};
use serde::{Deserialize, Serialize};
use serenity::{GuildId, MessageId};

use crate::{prelude::*, GuildEntity};
use crate::structures::session::Session;
use super::auth::Guilds;



#[derive(Clone)]
pub struct Guild(pub GuildInfo);

#[async_trait]
impl<S> FromRequestParts<S> for Guild
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(parts.extensions.get::<Self>().unwrap().clone())
    }
}


pub async fn server(
    State(state): State<AppState>,
    guilds: Guilds,
    Path(id): Path<GuildId>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse> {
    let user_guild = guilds.iter().cloned().find(|guild| guild.id == id);      
    
    if user_guild.is_none() || !state.cache.guilds().contains(&id){
        return Err(Error::GuildNotFound)
    }
    
    let guild = user_guild.unwrap();
    
    if !guild.permissions.administrator() {
        return Err(Error::AccessDenied)
    }

    GuildEntity::new(&id).find_or_create().await?;

    req.extensions_mut().insert(Guild(guild));
    

    Ok(next.run(req).await)
}
