use std::time::Duration;

use giveaway_manager::GiveawayManager;
use poise::serenity_prelude::{self as serenity};
use tokio::time::sleep;

mod commands;
mod giveaway_manager;
mod prelude;
use futures::lock::Mutex;
use prelude::*;
/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let servers_msg = ctx
        .cache()
        .guilds()
        .iter()
        .map(|g| g.name(ctx.cache()).unwrap())
        .collect::<Vec<_>>()
        .join(", ");
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.reply(response).await?;
    ctx.say(servers_msg).await?;
    Ok(())
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
    let token = "MTI0ODAyNDk4MzMwNTk4MTk2Mg.GMyrcS.zaWcvrrLizzWZ5nBdDUJtJNluRRa0EDpLRB_-U";
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), commands::start(), help()],
            event_handler: |ctx, event, framework, _data| {
                Box::pin(event_handler(ctx, event, framework))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    manager: Mutex::new(GiveawayManager::new()),
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
    if let Err(why) = client.start_shards(2).await {
        println!("Client error: {why:?}");
    }
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
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
        _ => {}
    }
    Ok(())
}

async fn perform_task(id: u32) {
    println!("Task {} executed!", id);
}
//
// #[tokio::main]
// async fn main() {
//
//     // Optionally, wait for all tasks to complete (not blocking other tasks)
//     // join_all(tasks).await;
// }
