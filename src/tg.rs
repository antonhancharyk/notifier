use teloxide::prelude::*;

pub async fn send(msg: String) {
    let bot = Bot::from_env();

    let ids = vec![457923379, 284411673];

    for id in ids {
        _ = bot.send_message(ChatId(id), msg.clone()).await
    }
}
