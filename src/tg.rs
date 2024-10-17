use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    Start,
    Status,
}

#[tokio::main]
pub async fn start() {
    let bot = Bot::from_env();
    let chat_ids: Arc<Mutex<HashSet<i64>>> =
        Arc::new(Mutex::new(vec![457923379].into_iter().collect()));

    Command::repl(bot, move |bot, msg, cmd| {
        let chat_ids_clone = chat_ids.clone();
        async move { answer(bot, msg, cmd, chat_ids_clone).await }
    })
    .await;
}

async fn answer(
    bot: Bot,
    msg: Message,
    cmd: Command,
    chat_ids: Arc<Mutex<HashSet<i64>>>,
) -> ResponseResult<()> {
    {
        let mut ids = chat_ids.lock().unwrap();
        ids.insert(msg.chat.id.0);
        println!("{}", msg.chat.id.0)
    }

    let start_msg = format!("Start");
    let status_msg = format!("Alive");

    match cmd {
        Command::Start => bot.send_message(msg.chat.id, start_msg).await?,
        Command::Status => bot.send_message(msg.chat.id, status_msg).await?,
    };

    Ok(())
}

pub async fn send(msg: String) {
    let bot = Bot::from_env();
    let chat_ids: Arc<Mutex<HashSet<i64>>> =
        Arc::new(Mutex::new(vec![457923379].into_iter().collect()));

    let ids = chat_ids.lock().unwrap();
    for &chat_id in ids.iter() {
        if let Err(err) = bot.send_message(ChatId(chat_id), msg.clone()).await {
            eprintln!("Failed to send Telegram message: {}", err);
        }
    }
}
