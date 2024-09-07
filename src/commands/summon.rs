use serenity::{
    builder::CreateMessage,
    framework::standard::{
        CommandResult,
        macros::command,
    },
    model::{
        channel::Message,
        user::User,
    },
    prelude::Context,
    utils::MessageBuilder,
};

#[command]
async fn summon(ctx: &Context, msg: &Message) -> CommandResult {
    if !msg.mentions.is_empty() {
        for user in &msg.mentions {
            if !user.bot {
                let nick = User::nick_in(&user, &ctx.http, &msg.guild_id.unwrap()).await;
                let message = CreateMessage::new().content("Get on stink ass");
                let dm_user = User::dm(&user, &ctx.http, message).await;
                let response = MessageBuilder::new()
                    .push("Summoning ")
                    .push_bold_safe(nick.unwrap())
                    .push("...")
                    .build();

                msg.channel_id.say(&ctx.http, response).await?;
            } else {
                // Error for summoning bots
                let response = MessageBuilder::new()
                    .push("Failed to summon ")
                    .push_bold_safe(&user.name)
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

    return Ok(())
}
