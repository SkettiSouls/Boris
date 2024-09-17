#![allow(deprecated)]
mod commands;

use std::{collections::HashSet, env, sync::Arc};

use serenity::{
    async_trait,
    framework::{
        standard::{macros::group, Configuration},
        StandardFramework,
    },
    gateway::ShardManager,
    model::channel::Message,
    model::gateway::Ready,
    prelude::{Client, Context, EventHandler, GatewayIntents, TypeMapKey},
};

use crate::commands::{clear::*, hooks::*, log::*, summon::*};

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name)
    }

    async fn message(&self, _ctx: Context, msg: Message) {
        log_dms(&msg).await
    }
}

#[group]
#[commands(summon, clear, test_clear)]
struct General;

#[tokio::main]
async fn main() {
    // Get environment variables from crate/.env
    dotenv::dotenv().expect("Failed to load .env file.");

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = serenity::http::Http::new(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(owner) = &info.owner {
                owners.insert(owner.id);
            }

            (owners, info.id)
        }

        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .before(before)
        .after(after)
        .group(&GENERAL_GROUP);

    framework.configure(Configuration::new().owners(owners).prefix("!"));

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.shutdown_all().await;
    });

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
