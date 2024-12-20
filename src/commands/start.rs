use std::{marker::PhantomData, str::FromStr, time::Duration};

use axum::async_trait;
use poise::PopArgumentHack;
use prisma_client::db::guild::entry_type;

use crate::{giveaway::options::GiveawayOptions, prelude::*};



#[derive(poise::ChoiceParameter, PartialEq)]
pub enum EntryType {
    #[name = "reaction"]
    Reaction,
    // A choice can have multiple names
    #[name = "button"]
    Button,
}

/// Create a giveaway command with prize, winners, and timer as arguments
#[poise::command(slash_command, prefix_command, category = "Giveaway")]
pub async fn start(
    ctx: Context<'_>,
    #[description = "Choose a prize"] prize: String,
    #[description = "number of winners"] winners: u32,
    #[description = "timer"] timer: String,
    #[description = "type"] entry_type: Option<EntryType>,
) -> Result<(), Error> {
    // check winners if is 0
    if winners == 0 {
        ctx.reply("Number of winners cannot be 0").await?;
        return Ok(());
    }

    let Ok(timer) = parse_duration::parse(&timer) else {
        ctx.reply("Invalid time format").await?;
        return Ok(());
    };

    // check if timer is less than 1 minute and greater than a week
    // if timer < Duration::from_secs(60) || timer > Duration::from_secs(60 * 60 * 24 * 7) {
    if timer < Duration::from_secs(2) || timer > Duration::from_secs(60 * 60 * 24 * 7) {
        ctx.reply("Timer must be between 1 minute and 1 week")
            .await?;
        return Ok(());
    }
    ctx.data()
        .manager
        .create(&ctx, GiveawayOptions::from_ctx(&ctx, prize, winners, timer, entry_type).await)
        .await?;
    Ok(())
}
