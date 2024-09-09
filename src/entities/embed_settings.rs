
use prisma_client::db::{embed_settings, PrismaClient};
use serenity::GuildId;

use crate::get_prisma;

use crate::prelude::*;
use crate::transformers::UpdateEmbed;
#[derive(Debug)]
pub struct EmbedSettingsEntity {
    prisma: &'static PrismaClient,
}

impl Default for EmbedSettingsEntity {
    fn default() -> Self {
        Self {
            prisma: get_prisma(),
        }
    }
}

impl EmbedSettingsEntity {
    pub fn new() -> Self {
        Self::default()
    }
    pub async fn create(&self) -> Result<embed_settings::Data, Error> {
        Ok(self.prisma.embed_settings().create(vec![]).exec().await?)
    }
    pub async fn update(&self, id: String, embed: UpdateEmbed) -> Result<()> {
        self.prisma
            .embed_settings()
            .update(
                embed_settings::UniqueWhereParam::IdEquals(id),
                vec![
                    embed_settings::color::set(embed.color),
                    embed_settings::title::set(embed.title),
                    embed_settings::description::set(embed.description),
                ],
            )
            .exec()
            .await
            .unwrap();
        Ok(())   
    }
}
