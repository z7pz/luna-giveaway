use serenity::MessageId;

use crate::prelude::*;

/// Create a giveaway command with prize, winners, and timer as arguments
#[poise::command(slash_command, prefix_command, category = "Giveaway")]
pub async fn end(
    ctx: Context<'_>,
    #[description = "Giveaway message id"] message_id: MessageId,
) -> Result<(), Error> {
    if let Some(giveaway) = ctx.data().manager.giveaways.get_mut(&message_id) {
        // TODO move this into the manager so it can be done in the background and remove giveaway cache and task
        giveaway
            .lock()
            .await
            .end(ctx.serenity_context().http.clone())
            .await?;
    }
    Ok(())
}
