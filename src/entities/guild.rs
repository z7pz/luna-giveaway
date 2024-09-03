use prisma_client::db::{embed_settings, guild, PrismaClient};
use serenity::GuildId;

use crate::get_prisma;

use crate::prelude::*;

use super::embed_settings::EmbedSettingsEntity;
#[derive(Debug)]
pub struct GuildEntity<'a> {
    guild_id: &'a GuildId,
    prisma: &'static PrismaClient,
    embed_settings_entity: EmbedSettingsEntity,
}

impl<'a> GuildEntity<'a> {
    pub fn new(guild_id: &'a GuildId) -> Self {
        Self {guild_id,
            prisma: get_prisma(),
            embed_settings_entity: EmbedSettingsEntity::new()
        }
    }
    pub async fn find_or_create(&self) -> Result<guild::Data, Error> {
        let data = self.find_one().await?;
        if let Some(data) = data {
            return Ok(data);
        }
        Ok(self.create().await?)
    }
    pub async fn update_commands(&self, disabled_commands: Vec<String>) -> Result<(), Error> {
        self.find_or_create().await?;
        self.prisma.guild().update(guild::UniqueWhereParam::IdEquals(self.guild_id.clone().into()), vec![guild::disabled_commands::set(disabled_commands)]).exec().await?;
        Ok(())
    }
    pub async fn find_one(&self) -> Result<Option<guild::Data>, Error> {
        Ok(self.prisma.guild().find_first(vec![guild::id::equals(self.guild_id.clone().into())]).with(guild::start_embed_settings::fetch()).with(guild::end_embed_settings::fetch()).exec().await?)
    }
    pub async fn find_one_with_giveaways(&self) -> Result<Option<guild::Data>, Error> {
        Ok(self.prisma.guild().find_first(vec![guild::id::equals(self.guild_id.clone().into())]).with(guild::start_embed_settings::fetch()).with(guild::end_embed_settings::fetch()).with(guild::giveaways::fetch(vec![])).exec().await?)
    }
    pub async fn create(&self) -> Result<guild::Data, Error> {
        let start = self.embed_settings_entity.create().await?;
        let end = self.embed_settings_entity.create().await?;
        Ok(self
            .prisma
            .guild()
            .create(
                self.guild_id.clone().into(),
                embed_settings::UniqueWhereParam::IdEquals(start.id),
                embed_settings::UniqueWhereParam::IdEquals(end.id),
                vec![],
            )
            .exec()
            .await?)
    }
}
