use teloxide::prelude::*;
use teloxide::types::{ChatMemberStatus, UserId, MessageKind, Message};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
pretty_env_logger::init();
println!("BOT_TOKEN: {:?}", env::var("BOT_TOKEN"));
    println!("bot is ready!");
    let bot = Bot::from_env();

    let channel = env::var("CHANNEL_ID").expect("CHANNEL_ID must be set");
    let group_link = env::var("GROUP_LINK").expect("GROUP_LINK must be set");

    teloxide::repl(bot.clone(), move |msg: Message| {
        let bot = bot.clone();
        let channel = channel.clone();
        let group_link = group_link.clone();

        async move {
            if is_start_command(&msg) {
                let user_id = UserId(msg.from().unwrap().id.0);

                let chat_member = bot.get_chat_member(channel.clone(), user_id).send().await;

                match chat_member {
                    Ok(member) => {
                        match member.status() {
                            ChatMemberStatus::Member
                            | ChatMemberStatus::Administrator
                            | ChatMemberStatus::Owner => {
                                bot.send_message(msg.chat.id, format!(
                                    " Вы участник канала! Вот ссылка на группу:\n{}",
                                    group_link
                                ))
                                .await?;
                            }
                            _ => {
                                bot.send_message(
                                    msg.chat.id,
                                    "Вы не являетесь участником канала.",
                                )
                                .await?;
                            }
                        }
                    }
                    Err(_) => {
                        bot.send_message(
                            msg.chat.id,
                            "Не удалось проверить участие. Возможно, вы не подписаны или канал слишком приватен.",
                        )
                        .await?;
                    }
                }
            }

            Ok(())
        }
    })
    .await;
}

fn is_start_command(message: &Message) -> bool {
    if let MessageKind::Common(common) = &message.kind {
        if let teloxide::types::MediaKind::Text(text) = &common.media_kind {
            return text.text == "/start";
        }
    }
    false
}
