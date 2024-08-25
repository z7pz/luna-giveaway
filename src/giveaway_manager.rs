#![allow(unused)]
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use futures::lock::Mutex;
use once_cell::sync::OnceCell;
use poise::serenity_prelude::{
    CacheHttp, ChannelId, CreateEmbed, CreateMessage, GuildId, Http, MessageId, UserId,
};
use serenity::Message;
use tokio::{sync::mpsc::Sender, time::sleep};

use crate::prelude::*;
#[derive(Debug)]
pub struct GiveawayArguments {
    pub prize: String,
    pub winners: u32,
    pub timer: Duration,
    pub host: String,
    pub channel_id: ChannelId,
    pub guild_id: GuildId,
    pub entries: Vec<UserId>,
    pub starts_at: Duration,
    pub ends_at: Duration,
    pub is_ended: bool,
}

impl GiveawayArguments {
    pub fn new(ctx: &Context<'_>, prize: String, winners: u32, timer: Duration) -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        Self {
            prize,
            winners,
            timer,
            host: ctx.author().to_string(),
            channel_id: ctx.channel_id().into(),
            guild_id: ctx.guild_id().expect("Failed to get the guild id").into(), // WARN unwrap
            entries: Vec::new(),
            starts_at: since_the_epoch,
            ends_at: since_the_epoch + timer,
            is_ended: false,
        }
    }
}
#[derive(Debug)]
pub struct Giveaway {
    pub message_id: MessageId,
    pub args: GiveawayArguments,
}
#[derive(Debug)]

pub struct GiveawayJob {
    pub giveaway: Arc<Giveaway>,
    pub task: tokio::task::JoinHandle<()>,
}

pub struct GiveawayManager {
    pub tx: Sender<Arc<Giveaway>>,
    pub giveaways: Arc<Mutex<HashMap<MessageId, Giveaway>>>,
    pub tasks: Arc<Mutex<HashMap<MessageId, GiveawayJob>>>,
}
impl GiveawayManager {
    pub async fn new(tx: Sender<Arc<Giveaway>>) -> Self {
        let giveaways = Arc::new(Mutex::new(HashMap::new()));
        let tasks = Arc::new(Mutex::new(HashMap::new()));

        Self {
            tx,
            giveaways,
            tasks,
        }
    }
    pub async fn create(&self, ctx: &Context<'_>, args: GiveawayArguments) -> Result<(), Error> {
        let tx = self.tx.clone();

        let giveaways = self.giveaways.clone();

        // sending giveaway start message
        let message = self
            .send_start_message(ctx.serenity_context().http.clone(), ctx.channel_id(), &args)
            .await?;

        let giveaway = Arc::new(Giveaway {
            message_id: message.id,
            args,
        });

        // spawn a task to send giveaway to the tx channel when giveaway ends
        let g = giveaway.clone();
        let task = tokio::spawn(async move {
            // sleep(g.args.timer).await;
            {
                let mut giveaways = giveaways.lock().await;
                giveaways.remove(&g.message_id);
                println!("{}", giveaways.len())
            }
            if let Err(error) = tx.send(g).await {
                println!("Error: {:?}", error);
            };
        });

        self.tasks
            .lock()
            .await
            .insert(message.id, GiveawayJob { giveaway, task });
        Ok(())
    }
    pub fn end(&self) {}
    pub fn reroll(&self) {}
}
/// Start message methods
pub trait StartMessage {
    fn get_description(&self, args: &GiveawayArguments) -> String {
        format!(
            "Prize: {}\nEntries: {}\nWinners: {}\nTime: <t:{3}:R> <t:{3}>",
            args.prize,
            args.entries.len(),
            args.winners,
            args.ends_at.as_secs(),
        )
    }
    fn start_embed(&self, args: &GiveawayArguments) -> CreateEmbed {
        let embed = CreateEmbed::default()
            .title("Giveaway")
            .description(self.get_description(&args))
            .color(0x00ff00);

        embed
    }
    fn start_messsage(&self, args: &GiveawayArguments) -> CreateMessage {
        CreateMessage::new().embed(self.start_embed(args))
    }
    async fn send_start_message(
        &self,
        http: Arc<Http>,
        channel_id: ChannelId,
        args: &GiveawayArguments,
    ) -> Result<Message, Error> {
        Ok(channel_id
            .send_message(http, self.start_messsage(args))
            .await?)
    }
}

impl StartMessage for GiveawayManager {}
