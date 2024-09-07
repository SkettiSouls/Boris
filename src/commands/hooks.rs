use serenity::{
    framework::standard::{
        CommandResult,
        macros::hook,
    },
    model::channel::Message,
    prelude::Context,
};

#[hook]
pub async fn before(_ctx: &Context, msg: &Message, cmd: &str ) -> bool {
    println!("Received command `{}` from {}", cmd, msg.author.name);
    true
}

#[hook]
pub async fn after(_ctx: &Context, _msg: &Message, cmd: &str, result: CommandResult) {
    match result {
        Ok(()) => println!("Command `{cmd}` succeeded"),
        Err(why) => println!("Command `{cmd}` failed with error {why:?}")
    }
}

