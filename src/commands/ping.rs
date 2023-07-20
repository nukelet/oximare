use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::prelude::Context;
use serenity::model::channel::Message;

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "pong").await?;
    Ok(())
}