use std::error::Error;
use sea_orm::{Database, DatabaseConnection};
use entities::{User, Category, Track};
use teloxide::{prelude::*, utils::command::BotCommands, types::InputFile};

#[tokio::main]
async fn main() {
    let bot = Bot::from_env().auto_send();
    let env = std::env::var("DATABASE_URL").unwrap();
    let database = Database::connect(&env).await.unwrap();
    let handler = Update::filter_message()
        .branch( dptree::entry()
            .filter_command::<Command>()
            .endpoint(answer)
        );
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![database])
        .build()
        .setup_ctrlc_handler()
        .dispatch().await;
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "start")]
    Start,
    #[command(description = "create category for your time tracks. name should be unique")]
    CreateCategory (String),
    ShowCategories
}

async fn answer(
    bot: AutoSend<Bot>,
    database: DatabaseConnection,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let user_id = message.chat.id.0 as i32;
    match command {
        Command::Start => {
            User::new(&database, user_id).await?;
            let sticker = InputFile::file_id("");
            bot.send_sticker(ChatId (user_id as i64), sticker).await?;
        }
        Command::CreateCategory (name) => {
            Category::new(&database, user_id, name).await?;
            let sticker = InputFile::file_id("");
            bot.send_sticker(ChatId (user_id as i64), sticker).await?;
        }
        Command::ShowCategories => {
            let categories = Category::users_categories(&database, user_id).await?;
        }
    }
    Ok(())
}
