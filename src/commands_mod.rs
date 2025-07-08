use teloxide::types::{MessageKind, Message};


pub fn is_start_command(message: &Message) -> bool {
    if let MessageKind::Common(common) = &message.kind {
        if let teloxide::types::MediaKind::Text(text) = &common.media_kind {
            return text.text == "/start";
        }
    }
    false
}