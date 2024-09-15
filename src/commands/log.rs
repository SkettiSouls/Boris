use serenity::model::{channel::Message, id::UserId};

pub async fn log_dms(msg: &Message) {
    // TODO: Get self user ID dynamically
    let is_self = msg.author.bot && msg.author.id == UserId::from(1281203381595275348);
    if msg.is_private() && !is_self {
        println!(
            "DM received from user '{0}:{1}', with content:",
            msg.author.name, msg.author
        );
        println!("{}", msg.content)
    }
}
