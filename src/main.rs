mod cmd;
use cmd::{channels::*, invite::*};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serenity::async_trait;
use serenity::framework::standard::{
    help_commands,
    macros::{group, help},
    Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::framework::StandardFramework;
use serenity::model::{channel::Message, gateway::Ready, id::UserId};
use serenity::prelude::{Client, Context, EventHandler};
use std::{collections::HashSet, fs::File, io::BufReader, usize};

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[help]
#[individual_command_tip = "help"]
#[strikethrough_commands_tip_in_guild = ""]
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[group]
#[description("invite bot")]
#[summary("")]
#[commands(invite, all_channels)]
struct General;

#[derive(Serialize, Deserialize)]
struct Token {
    token: String,
}

fn get_token(file_name: &str) -> Result<String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let t: Token = serde_json::from_reader(reader).unwrap();
    Ok(t.token)
}

#[tokio::main]
async fn main() {
    let token = get_token("config.json").expect("Err: token not found");
    let framework = StandardFramework::new()
        // TODO: prefix
        .configure(|c| c.prefix("\\"))
        .help(&MY_HELP)
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err: creating client");

    if let Err(why) = client.start().await {
        println!("Client Err: {:?}", why);
    }
}
