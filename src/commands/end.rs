use serenity::MessageId;

use crate::{prelude::*};

/// Create a giveaway command with prize, winners, and timer as arguments
#[poise::command(slash_command, prefix_command)]
pub async fn end(
    ctx: Context<'_>,
    #[description = "Giveaway message id"] message_id: MessageId,
) -> Result<(), Error> {

    Ok(())
}
