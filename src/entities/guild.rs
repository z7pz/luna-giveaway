use prisma_client::db::{embed_settings, guild, PrismaClient};
use serenity::GuildId;

use crate::get_prisma;

use crate::prelude::*;

use super::embed_settings::EmbedSettingsEntity;
#[derive(Debug)]
pub struct GuildEntity {
    prisma: &'static PrismaClient,
    embed_settings_entity: EmbedSettingsEntity,
}

impl Default for GuildEntity {
    fn default() -> Self {
        Self {
            prisma: get_prisma(),
            embed_settings_entity: EmbedSettingsEntity::new(),
        }
    }
}

impl GuildEntity {
    pub fn new() -> Self {
        Self::default()
    }
	pub async fn test(&self) {

	}
    pub async fn find_or_create(&self, guild_id: GuildId) -> Result<guild::Data, Error> {
        Ok(self.prisma.guild().upsert(
            guild::UniqueWhereParam::IdEquals(guild_id.into()),
            (
                guild_id.into(),
                embed_settings::UniqueWhereParam::IdEquals(
                    self.embed_settings_entity.create().await?.id,
                ),
                embed_settings::UniqueWhereParam::IdEquals(
                    self.embed_settings_entity.create().await?.id,
                ),
                vec![],
            ),
            vec![],
        ).exec().await?)
    }

    pub async fn create(&self, guild_id: GuildId) -> Result<guild::Data, Error> {
        let start = self.embed_settings_entity.create().await?;
        let end = self.embed_settings_entity.create().await?;
        Ok(self
            .prisma
            .guild()
            .create(
                guild_id.into(),
                embed_settings::UniqueWhereParam::IdEquals(start.id),
                embed_settings::UniqueWhereParam::IdEquals(end.id),
                vec![],
            )
            .exec()
            .await?)
    }
    pub fn end(&self) -> Result<(), Error> {
        // end giveaway
        Ok(())
    }
}
