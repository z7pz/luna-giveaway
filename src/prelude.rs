use std::sync::Arc;

use crate::giveaway::manager::GiveawayManager;
#[derive(Clone)]
pub struct Data {
    pub manager: GiveawayManager,
    pub commands: Vec<String>,
    pub prisma: &'static PrismaClient,
}
#[derive(Clone)]

pub struct AppState {
    pub data: Data,
    pub cache: Arc<Cache>,
}
pub type Error = crate::error::Error;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Result<T, E = Error> = std::result::Result<T, E>;
use poise::serenity_prelude::Cache;
pub use poise::serenity_prelude::{self as serenity};
use prisma_client::db::PrismaClient;
