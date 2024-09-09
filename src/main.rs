#[macro_use]
extern crate lazy_static;
use axum::Router;
use config::{DEFAULT_PREFIX, DISCORD_TOKEN, PORT};
use entities::*;
use giveaway::manager::GiveawayManager;
use http::{HeaderValue, Method};
use once_cell::sync::OnceCell;
use prisma_client::db::{EntryType, PrismaClient};
use tokio::{net::TcpListener, sync::mpsc};
use poise::{
    serenity_prelude::{
        self as serenity, CreateInteractionResponse, CreateInteractionResponseMessage,
    },
    PrefixFrameworkOptions,
};
use prelude::*;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use transformers::CommandResponse;
use std::{ops::Deref, time::Duration};
use tokio::time::sleep;

mod commands;
mod config;
mod entities;
mod error;
mod giveaway;
mod middlewares;
mod prelude;
mod routes;
mod structures;
mod utils;
mod transformers;



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
        command.as_deref(),
        poise::builtins::HelpConfiguration::default(),
    )
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let commands = vec![
        commands::end(),
        commands::start(),
        commands::reroll(),
        help(),
    ];

    let prisma = PrismaClient::_builder()
        .build()
        .await
        .expect("Failed to create Prisma client");

    PRISMA.set(prisma).expect("Failed to set Prisma client");

    let intents = serenity::GatewayIntents::all();
    let (tx, mut rx) = mpsc::channel(100);
    let manager = GiveawayManager::new(tx).await;
    let data = Data {
        manager,
        prisma: get_prisma(),
        commands: commands
            .iter()
            .filter(|f|f.category.as_ref().map(|c| !["Admin"].contains(&c.as_str())).unwrap_or(false))
            .map(|f| CommandResponse::from_command(f))
            .collect(),
    };
    let router_data = data.clone();

    // region: Setup framework
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            prefix_options: PrefixFrameworkOptions {
                dynamic_prefix: Some(|ctx| {
                    Box::pin(async move {
                        if let Some(id) = ctx.guild_id {
                            if let Ok(guild) = GuildEntity::new(&id).find_or_create().await {
                                Ok(Some(guild.prefix))
                            } else {
                                Ok(None)
                            }
                        } else {
                            Ok(None)
                        }
                    })
                }),
                prefix: Some(DEFAULT_PREFIX.deref().clone()),
                ..Default::default()
            },
            command_check: Some(|ctx| {
                Box::pin(async move {
                    if ctx.author().bot {
                        return Ok(false);
                    }
                    println!("Checking command: {:?}", ctx.command().name);

                    if let Some(id) = ctx.guild_id() {
                        let entity = GuildEntity::new(&id);
                        let guild = entity.find_or_create().await?;
                        if guild.disabled_commands.contains(&ctx.command().name) {
                            return Ok(false);
                        };
                        if let Some(member) = ctx.author_member().await {
                            // check if user has admin permissions
                            let is_admin = member
                                .permissions(ctx.cache())
                                .map(|p| p.administrator())
                                .unwrap_or(false);
                            if is_admin {
                                return Ok(true);
                            }
                            // check if user has roles to play to command
                            let has_role = member
                                .roles
                                .iter()
                                .any(|role| return guild.creator_roles.contains(&role.to_string()));
                            if has_role {
                                return Ok(true);
                            }
                            return Ok(false);
                        } else {
                            return Ok(false);
                        }
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
                data.manager.hydrate(ctx.http.clone()).await;
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .build();
    // endregion: setup framework


    // region: setup serenity client
    let client = serenity::ClientBuilder::new(&*DISCORD_TOKEN, intents)
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
    // endregion: setup serenity client

    let cache_http = client.http.clone();
    let manager_job = tokio::spawn(async move {
        while let Some(giveaway) = rx.recv().await {
            let cache_http = cache_http.clone();
            // TODO handle error
            let _ = giveaway.lock().await.end(cache_http).await;
        }
    });

    let app_state = AppState {
        data: router_data,
        cache: client.cache.clone(),
        http: client.http.clone()
    };
    // region: Router
    let app = routes::mount(Router::new(), app_state.clone())
        .layer(CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(AllowOrigin::exact("http://127.0.0.1:3000".parse().unwrap())))
        .with_state(app_state);
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    tokio::spawn(async move {
        axum::serve(
            TcpListener::bind(format!("127.0.0.1:{}", *PORT))
                .await
                .expect("error binding"),
            app.into_make_service(),
        )
        .await
        .unwrap();
    });
    // endregion: router

    let client_job = tokio::spawn(async move {
        if let Err(why) = client.start().await {
            println!("Client error: {why:?}");
        }
    });

    client_job.await.unwrap();
    manager_job.await.unwrap();
}

// region: Event handler
async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::ReactionAdd { add_reaction } => {
            println!("{:?}", add_reaction.guild_id);
            // chekc if reaction on embed
            if let Some(giveaway) = data.manager.giveaways.get(&add_reaction.message_id) {
                let mut giveaway = giveaway.lock().await;
                // check if the giveaway is reaction type
                println!("Reaction add");
                if giveaway.options.guild.entry_type == EntryType::Reaction {
                    println!("{} {}", giveaway.options.guild.reaction, add_reaction.emoji.to_string());
                    if giveaway.options.guild.reaction == add_reaction.emoji.to_string() {
                        println!("Adding entry: {:?}", add_reaction.user_id);
                        if let Some(id) = add_reaction.user_id {
                            giveaway.add_entry(id, ctx.http.clone()).await?;
                        } else {
                            println!("No user id");
                        }
                    } else {
                        println!("{}", add_reaction.emoji.as_data());
                        println!("Wrong reaction");
                    }
                }
            } else {
                println!("No giveaway found");
            }
        }
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
                if interaction.data.custom_id.as_str() != "giveaway" {
                    return Ok(());
                }
                let giveaway_id = interaction.message.id;
                println!("giveaway button");
                if let Some(giveaway) = data.manager.giveaways.get(&giveaway_id) {
                    println!("giveaway found");
                    if let Err(error) = giveaway
                        .lock()
                        .await
                        .add_entry(interaction.user.id, ctx.http.clone())
                        .await
                    {
                        interaction
                            .create_response(
                                ctx.http.clone(),
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::new()
                                        .content(error.to_string())
                                        .ephemeral(true),
                                ),
                            )
                            .await?;
                    }
                    interaction.defer(ctx.http.clone()).await?;
                };
            }
        }
        _ => {}
    }
    Ok(())
}
// endregion: event handler
