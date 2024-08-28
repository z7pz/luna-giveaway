use entities::{giveaway::GiveawayEntity, guild::GuildEntity};
use giveaway::manager::GiveawayManager;
use log::{info, trace};
use once_cell::sync::OnceCell;
use prisma_client::db::PrismaClient;
use tokio::sync::mpsc;

use poise::serenity_prelude::{self as serenity, EditMessage};
use std::time::Duration;
use tokio::time::sleep;

mod commands;
mod entities;
mod giveaway;

mod prelude;
use prelude::*;

pub static PRISMA: OnceCell<PrismaClient> = OnceCell::new();

fn get_prisma() -> &'static PrismaClient {
    PRISMA.get().expect("Prisma client not set")
}

/// If a command is specified, it will display information about that command
#[poise::command(slash_command, prefix_command)]
async fn help(
    ctx: Context<'_>,
    #[description = "Select a command"] command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_ref().map(|c| c.as_str()),
        poise::builtins::HelpConfiguration::default(),
    )
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let prisma = PrismaClient::_builder()
        .build()
        .await
        .expect("Failed to create Prisma client");
    println!("Prisma client creating...");

    PRISMA.set(prisma).expect("Failed to set Prisma client");

    println!("Prisma client created");

    let token = "MTI0ODAyNDk4MzMwNTk4MTk2Mg.GMyrcS.zaWcvrrLizzWZ5nBdDUJtJNluRRa0EDpLRB_-U";

    let intents = serenity::GatewayIntents::non_privileged();
    let (tx, mut rx) = mpsc::channel(100);
    let manager = GiveawayManager::new(tx).await;
    let data = Data { manager };
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::end(),
                commands::start(),
                commands::reroll(),
                help(),
            ],
            command_check: Some(|ctx| {
                Box::pin(async move {
                    println!("Checking command: {:?}", ctx.command().name);
                    if let Some(id) = ctx.guild_id() {
                        sleep(Duration::from_secs(1)).await;
                        GuildEntity::new().find_or_create(id).await?;
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                    
                })
            }),
            pre_command: |ctx| {
                Box::pin(async move {
                    println!("Command started: {:?}", ctx.command().name);
                })
            },
            post_command: |ctx| {
                Box::pin(async move {
                    println!("Command ended: {:?}", ctx.command().name);
                })
            },
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    let mut client = client.unwrap();

    let manager = client.shard_manager.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(30)).await;

            let shard_runners = manager.runners.lock().await;

            for (id, runner) in shard_runners.iter() {
                println!(
                    "Shard ID {} is {} with a latency of {:?}",
                    id, runner.stage, runner.latency,
                );
            }
        }
    });

    // Start two shards. Note that there is an ~5 second ratelimit period between when one shard
    // can start after another.
    let cache_http = client.http.clone();
    let manager_job = tokio::spawn(async move {
        while let Some(giveaway) = rx.recv().await {
            let cache_http = cache_http.clone();
            // TODO handle error
            let _ = giveaway.lock().await.end(cache_http).await;
        }
    });
    let client_job = tokio::spawn(async move {
        if let Err(why) = client.start().await {
            println!("Client error: {why:?}");
        }
    });

    client_job.await.unwrap();
    manager_job.await.unwrap();
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            if let Some(shard) = data_about_bot.shard {
                // Note that array index 0 is 0-indexed, while index 1 is 1-indexed.
                //
                // This may seem unintuitive, but it models Discord's behaviour.
                println!(
                    "{} is connected on shard {}/{}! {:#?} {} guilds in cache",
                    data_about_bot.user.name,
                    shard.id,
                    shard.total,
                    data_about_bot
                        .guilds
                        .iter()
                        .map(|g| g.id)
                        .collect::<Vec<_>>(),
                    ctx.cache.guilds().len()
                );
            }
        }
        serenity::FullEvent::InteractionCreate { interaction } => {
            if let Some(interaction) = interaction.as_message_component() {
                interaction.defer(ctx.http.clone()).await?;
                if interaction.data.custom_id.as_str() != "giveaway" {
                    return Ok(());
                }
                let giveaway_id = interaction.message.id;
                println!("giveaway button");
                if let Some(giveaway) = data.manager.giveaways.get(&giveaway_id) {
                    println!("giveaway found");
                    giveaway
                        .lock()
                        .await
                        .add_entry(interaction.user.id, ctx.http.clone())
                        .await?;
                };
            }
        }
        _ => {}
    }
    Ok(())
}
