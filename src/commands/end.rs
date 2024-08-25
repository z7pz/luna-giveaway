use serenity::MessageId;

use crate::{giveaway_manager::get_manager, prelude::*};

/// Create a giveaway command with prize, winners, and timer as arguments
#[poise::command(slash_command, prefix_command)]
pub async fn end(
    ctx: Context<'_>,
    #[description = "Giveaway message id"] message_id: MessageId,
) -> Result<(), Error> {
    // TODO get from db args
    // TODO create error handle for this as embeds
    let lock = get_manager().lock().await;
    let (giveaway, status) = if let Some(giveaway) = lock.cache.get(&message_id) {
        if giveaway.args.lock().await.is_ended {
            ctx.reply("Giveaway already ended").await?;
            (giveaway, false)
        } else {
            (giveaway, true)
        }
    } else {
        ctx.reply("Giveaway not found").await?;
        return Ok(());
    };
    if status {
        let mut giveaway = giveaway.clone();
        drop(lock);
        giveaway.end(ctx.http()).await;
    }

    Ok(())
}
