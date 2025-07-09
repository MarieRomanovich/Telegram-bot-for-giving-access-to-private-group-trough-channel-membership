use teloxide::prelude::*;
use teloxide::types::{ChatMemberStatus, UserId, Message};
use crate::commands_mod;

pub async fn responses (bot: Bot, channel: String, group_link: String) {

    teloxide::repl(bot.clone(), move |msg: Message| {
        let bot = bot.clone();
        let channel = channel.clone();
        let group_link = group_link.clone();


        async move {
            if commands_mod::is_start_command(&msg) { 
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
