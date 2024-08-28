use prisma_client::db::{embed_settings, giveaway, guild, PrismaClient};

use crate::get_prisma;

use crate::prelude::*;
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
}
