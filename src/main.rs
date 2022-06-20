#[macro_use]
extern crate log;
// use poise::serenity_prelude::{self as serenity, Channel, ChannelType, OnlineStatus};
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
    #[description = "who ?"] user1: serenity::User,
    #[description = "who ?"] user2: Option<serenity::User>,
    #[description = "who ?"] user3: Option<serenity::User>,
    #[description = "who ?"] user4: Option<serenity::User>,
    #[description = "where ?"] channel: Option<serenity::Channel>,
    #[description = "why ?"] description: Option<String>,
) -> Result<(), AppError> {
    let guild_id = ctx.guild_id().unwrap();

    // let online_members = ctx
    //     .guild()
    //     .unwrap()
    //     .members_with_status(OnlineStatus::Online);

    // let mut empty_member_voice_channels: Vec<serenity::GuildChannel> = vec![];
    // for channel in ctx.guild().unwrap().channels.values() {
    //     match channel {
    //         Channel::Guild(channel) => {
    //             // if channel.kind == ChannelType::Voice && channel.member_count.unwrap() == 0 {
    //             //     empty_member_voice_channels.push(channel.clone());
    //             // }
    //             // let a = &channel.name()[..9] == "Talk Room";
    //             let count = match channel.member_count {
    //                 Some(count) => count,
    //                 None => 0,
    //             };

    //             if channel.kind == ChannelType::Voice && count == 0 {
    //                 empty_member_voice_channels.push(channel.clone());
    //             }
    //         }
    //         _ => {}
    //     }
    // }
    // let channel_url = match channel {
    //     Some(channel) => format!("https://discord.com/channels/{}/{}", guild_id, channel.id()),
    //     None => match empty_member_voice_channels.len() {
    //         0 => String::from("(チャンネルが見つかりませんでした)"),
    //         _ => format!(
    //             "https://discord.com/channels/{}/{}",
    //             guild_id, empty_member_voice_channels[0].id
    //         ),
    //     },
    // };
    let channel_url = match channel {
        Some(channel) => format!("https://discord.com/channels/{}/{}", guild_id, channel.id()),
        None => String::from(""),
    };

    let mut users = vec![user1];
    match user2 {
        Some(user) => users.push(user),
        None => (),
    }
    match user3 {
        Some(user) => users.push(user),
        None => (),
    }
    match user4 {
        Some(user) => users.push(user),
        None => (),
    }

    let mention_string = users
        .iter()
        .map(|user| format!("<@{}>", user.id))
        .collect::<Vec<String>>()
        .join(" ");

    let _description = match description {
        Some(description) => {
            format!("{}\n\n", description)
        }
        None => String::from(""),
    };

    let msg = format!(
        "{}\n{}{}さんからボイスチャンネルに招待されました。\n{}",
        mention_string,
        _description,
        ctx.author().name,
        channel_url,
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
