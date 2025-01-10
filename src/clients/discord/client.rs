use super::{handler::Handler, Config};
use crate::{
    core::{Agent, Character},
    providers::completion::CompletionResponseEnum,
};
use log::{error, info};
use serenity::{prelude::Client as DiscordClient, prelude::*};
use std::env;
use tokio::time::sleep;

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
        let intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;

        let mut client = DiscordClient::builder(
            env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set"),
            intents,
        )
        .event_handler(Handler::new(
            self.character.clone(),
            Agent::new(self.completion_model.clone()),
            self.config.clone(),
        ))
        .await
        .expect("[DISCORD] Failed to create client");

        info!("[DISCORD] Starting client with 15s delay...");

        sleep(std::time::Duration::from_secs(15)).await;
        if let Err(why) = client.start().await {
            error!("[DISCORD] Client error: {why:?}");
        }
    }
}
