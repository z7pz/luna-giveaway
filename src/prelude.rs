use crate::giveaway::manager::GiveawayManager;
#[derive(Clone)]
pub struct Data {
    pub manager: GiveawayManager,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub use poise::serenity_prelude::{self as serenity};
