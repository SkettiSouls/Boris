use std::env;

use serenity::{
    async_trait,
    builder::CreateMessage,
    model::{
        channel::Message,
        gateway::Ready,
        user::User,
    },
    prelude::{
        Client,
        Context,
        EventHandler,
        GatewayIntents,
    },
    utils::MessageBuilder,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(why) = handler(self, ctx, msg).await {
            println!("Error sending message: {why:?}")
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name)
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file.");
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

async fn handler(handler: &Handler, ctx: Context, msg: Message) -> Result<(), Box<dyn std::error::Error>> {
    if msg.content.contains("!summon") {
        if !msg.mentions.is_empty() {
            for user in msg.mentions {
                if !user.bot {
                    let message = CreateMessage::new().content("Get on stink ass");
                    let dm_user = User::dm(&user, &ctx.http, message).await;
                    let response = MessageBuilder::new()
                        .push("Summoning ")
                        .push_bold_safe(user.name)
                        .push("...")
                        .build();

                    msg.channel_id.say(&ctx.http, response).await?;
                } else {
                    // Error for summoning bots
                    let response = MessageBuilder::new()
                        .push("Failed to summon ")
                        .push_bold_safe(user.name)
                        .push_line(":")
                        .push("Cannot summon beings without souls.")
                        .build();

                    msg.channel_id.say(&ctx.http, response).await?;
                }
            }
        } else {
            let response = MessageBuilder::new()
                .push("Please provide a user to summon.")
                .build();

            msg.channel_id.say(&ctx.http, response).await?;
        }
    }

    return Ok(())
}
