use crate::prelude::*;
use crate::{Context, Error};
use futures::lock::Mutex;
use serenity::{CacheHttp, ChannelId, EditMessage, GuildId, Http, Message, MessageId, UserId};

use rand::seq::SliceRandom;
use std::sync::Arc;
use std::time::Instant;
use std::vec;
use std::{
    collections::HashMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::time::sleep;

use once_cell::sync::OnceCell;

pub static MANAGER: OnceCell<Arc<Mutex<GiveawayManager>>> = OnceCell::new();

pub fn get_manager() -> &'static Arc<Mutex<GiveawayManager>> {
    MANAGER.get().unwrap()
}

/// Arguments required to create a giveaway.
#[derive(Clone)]
pub struct GiveawayArgs {
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
            is_ended: false,
        }
    }
    fn get_random_winners(&self) -> Vec<UserId> {
        let mut rng = rand::thread_rng();
        let winners = self.winners as usize;
        let mut entries = self.entries.clone();
        entries.shuffle(&mut rng);
        entries.truncate(winners);
        entries
    }
    pub async fn end(&mut self, http: impl CacheHttp, message_id: MessageId) {
        let manager = get_manager();
        let mut m = manager.lock().await;
        m.cache.remove(&message_id);
        drop(m);
        let http = Arc::new(http);
        let winners = self.get_random_winners();
        let builder = EditMessage::new()
            .add_embed(self.end_embed(winners.clone()))
            .components(vec![self.end_row()]);
        self.is_ended = true;
        let _ = self
            .channel_id
            .edit_message(http.clone(), message_id, builder)
            .await;
        let _ = self
            .channel_id
            .send_message(http, self.end_message(winners))
            .await;
    }
    fn start_row(&self) -> serenity::CreateActionRow {
        let button = serenity::CreateButton::new("giveaway")
            .label("Enter")
            .style(serenity::ButtonStyle::Primary);
        let row = serenity::CreateActionRow::Buttons(vec![button]);
        row
    }
    fn start_embed(&self) -> serenity::CreateEmbed {
        let embed = serenity::CreateEmbed::default()
            .title("Giveaway")
            .description(format!(
                "Prize: {}\nEntries: {}\nWinners: {}\nTime: <t:{3}:R> <t:{3}>",
                self.prize,
                self.entries.len(),
                self.winners,
                self.ends_at.as_secs(),
            ))
            .color(0x00ff00);

        embed
    }
    fn end_row(&self) -> serenity::CreateActionRow {
        // TODO add a button to redirect to the giveaway
        let button = serenity::CreateButton::new_link("https://google.com").label("Giveaway");
        let row = serenity::CreateActionRow::Buttons(vec![button]);
        row
    }
    fn end_embed(&self, winners: Vec<UserId>) -> serenity::CreateEmbed {
        println!("{:?}", winners);
        let w = winners
            .iter()
            .map(|c| format!("<@{}>", c))
            .collect::<Vec<String>>()
            .join(", ");
        let embed = serenity::CreateEmbed::default()
            .title("Giveaway Ended")
            .description(format!(
                "Prize: {}\nWinners: {}",
                self.prize,
                if w == "" { "No winners" } else { &w },
            ))
            .color((255, 0, 0)); //red

        embed
    }
    fn end_message(&self, winners: Vec<UserId>) -> serenity::CreateMessage {
        serenity::CreateMessage::new().content(format!(
            "{}",
            winners
                .iter()
                .map(|c| format!("<@{}>", c))
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }
}
#[derive(Clone, Debug)]
/// The object to manage a single giveaway.
pub struct Giveaway {
    pub message_id: MessageId,
    pub args: Arc<Mutex<GiveawayArgs>>,
    pub job: Arc<tokio::task::JoinHandle<()>>,
}

impl Giveaway {
    async fn start(args: Arc<Mutex<GiveawayArgs>>, http: Arc<Http>) -> Result<Self, Error> {
        let a = Arc::clone(&args);
        let lock = a.lock().await;
        let embed = lock.start_embed();
        let row = lock.start_row();
        // create something like performance.now() in js
        let now = Instant::now();
        let timer = lock.timer.clone();
        let giveaway_msg = {
            lock.channel_id.send_message(
                http.clone(),
                serenity::CreateMessage::new()
                    .add_embed(embed)
                    .components(vec![row]),
            )
        }
        .await?;
        let elapsed = now.elapsed();

        let id = giveaway_msg.id.clone();
        let a = Arc::clone(&args);

        let job = tokio::spawn(async move {
            sleep(timer - elapsed).await;
            { a.lock().await.end(http, id) }.await;
        });
        
        Ok(Giveaway {
            message_id: giveaway_msg.id,
            args,
            job: Arc::new(job),
        })
    }
    pub async fn end(&mut self, http: impl CacheHttp) {
        self.job.abort();
        let _ = { self.args.lock().await.end(http, self.message_id) }.await;
    }
    pub async fn add_entriy(&self, user: UserId, http: impl CacheHttp) {
        let mut args = self.args.lock().await;
        if args.entries.contains(&user) {
            return;
        }

        args.entries.push(user);
        drop(args);
        self.update(http).await;
        // TODO create a debounce
    }
    async fn update(&self, http: impl CacheHttp) {
        let args = self.args.lock().await;
        let embed = args.start_embed();
        let builder = EditMessage::new().add_embed(embed);
        let _ = args
            .channel_id
            .edit_message(http, self.message_id, builder)
            .await;
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
        let args = Arc::new(Mutex::new(GiveawayArgs::from_ctx(
            ctx,
            prize.clone(),
            winners.clone(),
            timer.clone(),
        )));

        let giveaway = Giveaway::start(args, http).await?;

        self.cache.insert(giveaway.message_id, giveaway);

        Ok(())
    }
    pub async fn end(&mut self, message_id: &MessageId, http: impl CacheHttp) {
        if let Some(giveaway) = self.cache.get_mut(message_id) {
            giveaway.end(http).await;
        }
    }
}
