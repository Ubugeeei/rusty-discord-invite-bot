#[macro_use]
extern crate log;
use poise::serenity_prelude as serenity;
use std::env;

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("{0}")]
    Serenity(#[from] poise::serenity::Error),
}

type Context<'a> = poise::Context<'a, (), AppError>;

#[poise::command(prefix_command, hide_in_help)]
async fn register(ctx: Context<'_>) -> Result<(), AppError> {
    poise::builtins::register_application_commands(ctx, false).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
async fn invite(
    ctx: Context<'_>,
    #[description = "who ?"] user: serenity::User,
    #[description = "where ?"] channel: serenity::Channel,
) -> Result<(), AppError> {
    let guild_id = ctx.guild_id().unwrap();
    let channel_url = format!("https://discord.com/channels/{}/{}", guild_id, channel.id());
    let msg = format!(
        "\n<@{}>\n{}さんからボイスチャンネルに招待されました。\n{}",
        user.id,
        ctx.author().name,
        channel_url
    );
    poise::say_reply(ctx, msg).await?;
    Ok(())
}

async fn on_error(error: poise::FrameworkError<'_, (), AppError>) {
    error!("{:?}", error);
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");

    let options = poise::FrameworkOptions {
        commands: vec![register(), invite()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("\\".to_string()),
            ..Default::default()
        },
        on_error: |err| Box::pin(on_error(err)),
        ..Default::default()
    };

    let framework = poise::Framework::build()
        .token(token)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .options(options)
        .user_data_setup(|_, _, _| Box::pin(async { Ok(()) }));

    framework.run().await.unwrap();
}
