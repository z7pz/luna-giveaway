use serenity::MessageId;

use crate::prelude::*;

/// Create a giveaway command with prize, winners, and timer as arguments
#[poise::command(slash_command, prefix_command)]
pub async fn end(
    ctx: Context<'_>,
    #[description = "Giveaway message id"] message_id: MessageId,
) -> Result<(), Error> {
    // TODO get from db args
    // TODO create error handle for this as embeds
    if let Some( giveaway) = ctx.data().manager.lock().await.cache.get_mut(&message_id) {
        if giveaway.args.lock().await.is_ended {
            ctx.reply("Giveaway already ended").await?;
        } else {
            giveaway.end(ctx.http()).await;
        }
    } else {
        ctx.reply("Giveaway not found").await?;
    }

    Ok(())
}
