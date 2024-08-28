use prisma_client::db::{giveaway, guild, PrismaClient};

use crate::get_prisma;

use crate::giveaway::giveaway::Giveaway;
use crate::prelude::*;
#[derive(Debug)]
pub struct GiveawayEntity {
    prisma: &'static PrismaClient,
}

impl Default for GiveawayEntity {
    fn default() -> Self {
        Self {
            prisma: get_prisma(),
        }
    }
}

impl GiveawayEntity {
    pub fn new() -> Self {
        Self::default()
    }
    pub async fn create(&self, giveaway: &Giveaway) -> Result<giveaway::Data, Error> {
        Ok(self.prisma.giveaway().create(
            giveaway.message_id.into(),
            giveaway.options.channel_id.into(),
            giveaway.options.prize.clone(),
            giveaway.options.starts_at.fixed_offset(),
            giveaway.options.ends_at.fixed_offset(),
            giveaway.options.winners as i32,
            guild::UniqueWhereParam::IdEquals(giveaway.options.guild_id.into()),
            vec![],
        ).exec().await?)
    }
    pub fn end(&self) -> Result<(), Error> {
        // end giveaway
        Ok(())
    }
}