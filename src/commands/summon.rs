use serenity::{
    builder::CreateMessage,
    framework::standard::{macros::command, CommandResult},
    model::{channel::Message, user::User},
    prelude::Context,
    utils::MessageBuilder,
};

use rand::seq::SliceRandom;

#[command]
async fn summon(ctx: &Context, msg: &Message) -> CommandResult {
    if !msg.mentions.is_empty() {
        for user in &msg.mentions {
            if !user.bot {
                let message_pool = [
                    "Get on stink ass",
                    "Holy shit man Humpty Dumpty just fell off the wall, I dont know what to do man bring a horse or something",
                    "IFRIT!!!! GET UP!!!!!!",
                    "My fellow americans, uhhh, get on the game (obama)",
                    r#"in your DMs. straight up "summoning it". and by "it" well, let's justr say.  My homie"#,
                    "Its time *vine boom*",
                    "PLEEEEEEEEEEEEEEEAAAAASE PLEASE PLEEASE PLEEEEEEEEEEEEEEEEEEEEEEEAAAAAAAAAAAASE",
                    "HELP IN GAIA",
                    "Wakey wakey",
                    "I may seem calm but in my mind I've killed you three times fucking join you dickhead", // Z
                ];

                let message = CreateMessage::new().content(
                    message_pool
                        .choose(&mut rand::thread_rng())
                        .unwrap()
                        .to_string(),
                );

                let nick = User::nick_in(&user, &ctx.http, &msg.guild_id.unwrap()).await;
                // TODO?: Add DM count
                let _dm_user = User::dm(&user, &ctx.http, message).await;
                let response = MessageBuilder::new()
                    .push("Summoning ")
                    .push_bold_safe(
                        nick.as_ref()
                            .or(user.global_name.as_ref())
                            .expect("Failed to get username"),
                    )
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

    return Ok(());
}
