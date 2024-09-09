use prisma_client::db::giveaway;
use prisma_client::db::{embed_settings, guild, PrismaClient};
use serenity::GuildId;

use crate::get_prisma;

use super::embed_settings::EmbedSettingsEntity;
use crate::prelude::*;
use crate::transformers::{UpdateEmbed, UpdateSettings};
#[derive(Debug)]
pub struct GuildEntity<'a> {
    guild_id: &'a GuildId,
    prisma: &'static PrismaClient,
    embed_settings_entity: EmbedSettingsEntity,
}

impl<'a> GuildEntity<'a> {
    pub fn new(guild_id: &'a GuildId) -> Self {
        Self {
            guild_id,
            prisma: get_prisma(),
            embed_settings_entity: EmbedSettingsEntity::new(),
        }
    }
    
    pub async fn find_or_create(&self) -> Result<guild::Data, Error> {
        let data = self.find_one().await?;
        if let Some(data) = data {
            return Ok(data);
        }
        Ok(self.create().await?)
    }
    pub async fn update(&self, settings: UpdateSettings) -> Result<()> {
        self.find_or_create().await.unwrap();
        let data = self.prisma
            .guild()
            .update(
                guild::UniqueWhereParam::IdEquals(self.guild_id.clone().into()),
                vec![
                    guild::prefix::set(settings.prefix),
                    guild::reaction::set(settings.reaction),
                    guild::entry_type::set(settings.entry_type),
                    guild::disabled_commands::set(settings.disabled_commands),
                    guild::creator_roles::set(settings.creator_roles.iter().map(|d| d.to_string()).collect::<Vec<_>>()),
                ],
            )
            .exec()
            .await
            .unwrap();
        self.embed_settings_entity.update(data.start_embed_settings_id, settings.start_embed_settings.clone()).await?;
        self.embed_settings_entity.update(data.end_embed_settings_id, settings.end_embed_settings.clone()).await?;
        Ok(())
    }
    pub async fn update_commands(&self, disabled_commands: Vec<String>) -> Result<(), Error> {
        self.find_or_create().await?;
        self.prisma
            .guild()
            .update(
                guild::UniqueWhereParam::IdEquals(self.guild_id.clone().into()),
                vec![guild::disabled_commands::set(disabled_commands)],
            )
            .exec()
            .await?;
        Ok(())
    }
    pub async fn find_one(&self) -> Result<Option<guild::Data>, Error> {
        Ok(self
            .prisma
            .guild()
            .find_first(vec![guild::id::equals(self.guild_id.clone().into())])
            .with(guild::start_embed_settings::fetch())
            .with(guild::end_embed_settings::fetch())
            .exec()
            .await?)
    }
    pub async fn find_one_with_giveaways(&self) -> Result<Option<guild::Data>, Error> {
        Ok(self
            .prisma
            .guild()
            .find_first(vec![guild::id::equals(self.guild_id.clone().into())])
            .with(guild::start_embed_settings::fetch())
            .with(guild::end_embed_settings::fetch())
            .with(
                guild::giveaways::fetch(vec![])
                    .with(giveaway::winners::fetch(vec![]))
                    .with(giveaway::entries::fetch(vec![])),
            )
            .exec()
            .await?)
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
