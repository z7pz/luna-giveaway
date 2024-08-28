use serenity::MessageId;

use crate::prelude::*;

/// Re-roll a giveaway
#[poise::command(slash_command, prefix_command)]
pub async fn reroll(
    ctx: Context<'_>,
    #[description = "Giveaway message id"] message_id: MessageId,
) -> Result<(), Error> {
    Ok(())
}
