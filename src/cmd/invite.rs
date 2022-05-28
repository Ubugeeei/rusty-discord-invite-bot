use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "invite meber for channel"]
async fn invite(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            format!("ボイスチャンネルに招待されました{} ", msg.author.mention()),
        )
        .await?;
    Ok(())
}
