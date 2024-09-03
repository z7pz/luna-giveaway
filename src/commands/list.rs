use crate::prelude::*;

/// List giveaways
#[poise::command(slash_command, prefix_command, category = "Giveaway")]
pub async fn list(
    ctx: Context<'_>,
    #[description = "Return all giveaways (ended giveaways included)"] all: bool,
) -> Result<(), Error> {
    Ok(())
}
