use crate::prelude::*;
use crate::{Context, Error};
use cron::Schedule;
use once_cell::sync::OnceCell;
use serenity::{CacheHttp, ChannelAction, ChannelId, GuildChannel, Http, Message, MessageId};
use std::sync::Arc;
use std::{
    collections::HashMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::time::sleep;



/// Arguments required to create a giveaway.
struct GiveawayArgs {
    pub prize: String,
    pub winners: u32,
    pub timer: Duration,
    pub host: String,
    pub channel_id: u64,
    pub guild_id: u64,
    pub entries: Vec<String>,
    pub starts_at: Duration,
    pub ends_at: Duration,
}

impl GiveawayArgs {
    fn from_ctx(ctx: &Context, prize: String, winners: u32, timer: Duration) -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        GiveawayArgs {
            prize,
            winners,
            timer,
            host: ctx.author().to_string(),
            channel_id: ctx.channel_id().into(),
            guild_id: ctx.guild_id().unwrap().into(), // WARN unwrap
            entries: Vec::new(),
            starts_at: since_the_epoch,
            ends_at: since_the_epoch + timer,
        }
    }
}

/// The object to manage a single giveaway.
pub struct Giveaway {
    pub args: GiveawayArgs,
    pub job: tokio::task::JoinHandle<()>,
}

impl Giveaway {
    async fn start(args: GiveawayArgs, http: Arc<Http>) -> (MessageId, Self) {

        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let embed = serenity::CreateEmbed::default()
            .title("Giveaway")
            .description(format!(
                "Prize: {}\nWinners: {}\nTime: <t:{2}:R> <t:{2}>",
                args.prize,
                args.winners,
                since_the_epoch.as_secs() + args.timer.as_secs()
            ))
            .color(serenity::Colour::ROSEWATER);

        let giveaway_msg = ChannelId::new(args.channel_id)
            .send_message(http.clone(), serenity::CreateMessage::new().add_embed(embed))
            .await
            .unwrap();
        let c = giveaway_msg.clone();
        
        let job = tokio::spawn(async move {
            sleep(args.timer).await;
            end(http, c).await;
        });

        (giveaway_msg.id, Giveaway { args, job })
    }
}

/// The object to manage multiple giveaways.
pub struct GiveawayManager {
    pub cache: HashMap<MessageId, Giveaway>,
}

impl GiveawayManager {
    pub fn new() -> Self {
        GiveawayManager {
            cache: HashMap::new(),
        }
    }

    pub async fn create_giveaway(
        &mut self,
        ctx: &Context<'_>,
        prize: String,
        winners: u32,
        timer: Duration,
    ) -> Result<(), Error> {
        let http = ctx.serenity_context().http.clone();

        let (giveaway_id, giveaway) = Giveaway::start(GiveawayArgs::from_ctx(
            ctx,
            prize.clone(),
            winners.clone(),
            timer.clone(),
        ), http).await;

        self.cache.insert(giveaway_id, giveaway);

        Ok(())
    }
}

async fn end(http: Arc<Http>, msg: Message) {
    msg.reply(http, "Giveaway ended!").await.unwrap();
}
