use super::{handler::Handler, Config};
use crate::{
    core::{Agent, Character},
    providers::completion::CompletionResponseEnum,
};
use log::info;
use std::sync::Arc;
use teloxide::prelude::{Bot as TelegramBot, *};
use tokio::time::sleep;

#[derive(Clone)]
pub struct Client<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum> + 'static,
{
    pub character: Character,
    pub completion_model: CM,
    pub config: Config,
}

impl<CM> Client<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
{
    pub async fn new(character: Character, completion_model: CM, config: Config) -> Self {
        Self {
            character,
            completion_model,
            config,
        }
    }

    pub async fn start(&self) {
        let bot =
            TelegramBot::new(std::env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN must be set"));

        let chat_handler = Handler::new(
            self.character.clone(),
            Agent::new(self.completion_model.clone()),
            self.config.clone(),
        );

        info!("[TELEGRAM] Starting client with 15s delay...");
        sleep(std::time::Duration::from_secs(15)).await;
        teloxide::repl(bot, {
            let chat_handler = Arc::new(chat_handler);

            move |message: Message, bot: Bot| {
                let chat_handler = Arc::clone(&chat_handler);

                async move { chat_handler.handle_message(message, bot).await }
            }
        })
        .await;
    }
}
