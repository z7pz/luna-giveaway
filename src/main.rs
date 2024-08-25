use std::{sync::Arc, time::Duration};

use giveaway_manager::{GiveawayManager, MANAGER};
use poise::serenity_prelude::{self as serenity};
use tokio::time::sleep;

mod commands;
mod giveaway_manager;
mod prelude;
use futures::lock::Mutex;
use prelude::*;

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
    let token = "MTI0ODAyNDk4MzMwNTk4MTk2Mg.GMyrcS.zaWcvrrLizzWZ5nBdDUJtJNluRRa0EDpLRB_-U";
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::end(), commands::start(),commands::reroll(), help()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            let manager = Arc::new(Mutex::new(GiveawayManager::new()));
            MANAGER.set(manager.clone()).unwrap();
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    manager,
                })
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
    // let mut tasks = vec![];
    // 
    //     for id in 1..=5000 {
    //         let duration = if id % 2 == 0 {
    //             Duration::from_secs(5) // 2 days
    //         } else {
    //             Duration::from_secs(2) // 7 days
    //         };
    // 
    //         let task = tokio::spawn(async move {
    //             sleep(duration).await;
    //             perform_task(id).await;
    //         });
    // 
    //         tasks.push(task);
    //     }

    // Start two shards. Note that there is an ~5 second ratelimit period between when one shard
    // can start after another.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
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
                    return Ok(())
                }
                let giveaway_id = interaction.message.id;
                if let Some(giveaway) = data.manager.lock().await.cache.get_mut(&giveaway_id) {
                    giveaway.add_entriy(interaction.user.id, &ctx.http).await;
                }
            }
        }
        _ => {}
    }
    Ok(())
}
