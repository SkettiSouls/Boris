use serenity::{
    builder::GetMessages,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
    utils::MessageBuilder,
};

#[command]
async fn clear(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let count: u8 = args
        .parse()
        .expect("Failed to parse count (likely non integer).");

    let builder = GetMessages::default().before(&msg.id).limit(count);

    // Delete the invoking message
    msg.delete(&ctx.http)
        .await
        .expect("Failed to delete invocation.");

    msg.channel_id
        .delete_messages(
            &ctx.http,
            msg.channel_id
                .messages(&ctx.http, builder)
                .await
                .unwrap()
                .iter()
                .map(|msg| msg.id),
        )
        .await
        .expect("Failed to delete messages");

    if count > 100 {
        let response = MessageBuilder::new()
            .push_line("Cannot remove more than 100 messages at a time.")
            .push("Cleared 100 messages.")
            .build();

        msg.channel_id.say(&ctx.http, response).await.expect("Failed to send message");
    }

    return Ok(());
}

#[command]
async fn test_clear(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let count: u8 = args
        .parse()
        .expect("Failed to parse count (likely non integer).");

    for i in 0..count {
        msg.channel_id
            .say(
                &ctx.http,
                serenity::utils::MessageBuilder::new()
                    .push(i.to_string())
                    .build(),
            )
            .await
            .expect("Failed to send test messages.");
    }

    msg.channel_id
        .say(
            &ctx.http,
            MessageBuilder::new()
                .push("Sent ")
                .push(count.to_string())
                .push(" messages.")
                .build()
        )
        .await
        .expect("Failed to send completion message.");

    return Ok(());
}
