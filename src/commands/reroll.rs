use serenity::MessageId;

use crate::prelude::*;

/// Re-roll a giveaway
#[poise::command(slash_command, prefix_command)]
pub async fn reroll(
    ctx: Context<'_>,
    #[description = "Giveaway message id"] message_id: MessageId,
) -> Result<(), Error> {
    // TODO get from db args
    // TODO create error handle for this as embeds
    if let Some(giveaway) = ctx.data().manager.lock().await.cache.get_mut(&message_id) {
        if giveaway.args.lock().await.is_ended {
            giveaway.end(ctx.http()).await;
			ctx.reply("Giveaway ended").await?;
		} else {
            ctx.reply("Giveaway not ended").await?;
        }
    } else {
        ctx.reply("Giveaway not found").await?;
    }

    Ok(())
}
