use std::{
    borrow::BorrowMut,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{giveaway_manager::GiveawayArguments, prelude::*};

/// Create a giveaway command with prize, winners, and timer as arguments
#[poise::command(slash_command, prefix_command)]
pub async fn start(
    ctx: Context<'_>,
    #[description = "Choose a prize"] prize: String,
    #[description = "number of winners"] winners: u32,
    #[description = "timer"] timer: String,
) -> Result<(), Error> {
    ctx.data().manager.create(&ctx, GiveawayArguments::new(&ctx, prize, winners, Duration::new(2, 0))).await?;
    Ok(())
}
