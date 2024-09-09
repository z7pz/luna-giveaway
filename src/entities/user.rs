use prisma_client::db::giveaway;
use prisma_client::db::user;
use prisma_client::db::{embed_settings, PrismaClient};
use serenity::GuildId;
use serenity::MessageId;
use serenity::UserId;

use crate::get_prisma;

use crate::prelude::*;
#[derive(Debug)]
pub struct UserEntity {
    id: UserId,
    prisma: &'static PrismaClient,
}

impl UserEntity {
    pub fn new(id: UserId) -> Self {
        Self {
            id,
            prisma: get_prisma(),
        }
    }
    pub async fn join_giveaway(&self, giveaway_id: &MessageId) -> Result<user::Data, Error> {
        Ok(self
            .prisma
            .user()
            .upsert(
                user::id::equals(self.id.clone().into()),
                (self.id.clone().into(), vec![user::giveaways::connect(vec![
                    giveaway::message_id::equals(giveaway_id.clone().into()),
                ])]),
                vec![user::giveaways::connect(vec![
                    giveaway::message_id::equals(giveaway_id.clone().into()),
                ])],
            )
            .exec()
            .await?)
    }
}
