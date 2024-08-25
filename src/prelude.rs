
use std::sync::Arc;

use crate::giveaway_manager::GiveawayManager;
pub struct Data {
	pub manager: Arc<Mutex<GiveawayManager>>,
} 
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
use futures::lock::Mutex;
pub use poise::serenity_prelude::{self as serenity};

