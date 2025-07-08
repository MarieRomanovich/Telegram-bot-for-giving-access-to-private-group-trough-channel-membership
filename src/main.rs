use teloxide::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::bot_responses::responses;
mod commands_mod;
mod bot_responses;

#[tokio::main]
async fn main() {
    dotenv().ok();
pretty_env_logger::init();
//println!("BOT_TOKEN: {:?}", env::var("BOT_TOKEN"));
    let bot = Bot::from_env();

    let channel = env::var("CHANNEL_ID").expect("CHANNEL_ID must be set");
    let group_link = env::var("GROUP_LINK").expect("GROUP_LINK must be set");

    responses(bot, channel, group_link).await
}


