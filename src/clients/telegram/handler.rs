use crate::{
    core::{
        Agent, Character,
        CharacterTrait::{Adjectives, Inspirations, Lore, Styles},
    },
    providers::completion::CompletionResponseEnum,
};
use log::{debug, error, info};
use rig::completion::Message as RigMessage;
use std::collections::HashMap;
use std::sync::Arc;
use teloxide::prelude::*;
use tokio::sync::Mutex;

use super::Config;

pub struct Handler<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
{
    agent: Agent<CM>,
    character: Character,
    config: Config,
    bot_history: Arc<Mutex<HashMap<i64, Vec<RigMessage>>>>,
    chat_history: Arc<Mutex<HashMap<i64, Vec<String>>>>,
}

impl<CM> Handler<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
{
    const BOT_HISTORY_SIZE: usize = 10;

    pub fn new(character: Character, agent: Agent<CM>, config: Config) -> Self {
        Self {
            character,
            agent,
            config,
            bot_history: Arc::new(Mutex::new(HashMap::new())),
            chat_history: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn handle_message(
        &self,
        message: Message,
        bot: Bot,
    ) -> Result<(), teloxide::RequestError> {
        if rand::random::<f32>() < self.config.selection_rate {
            if let Some(text) = message.text() {
                let previous_entries = {
                    let mut cache = self.chat_history.lock().await;
                    let chat_id = message.chat.id.0;

                    let entry = cache.entry(chat_id).or_insert_with(Vec::new);
                    let entries = entry.clone();

                    entry.push(format!(
                        "@{}: {}",
                        message.from.as_ref().map_or("unknown".to_string(), |u| {
                            u.username.as_deref().unwrap_or("unknown").to_string()
                        }),
                        text
                    ));
                    if entry.len() > self.config.surrounding_messages as usize {
                        entry.remove(0);
                    }

                    entries
                };

                let alias = match bot.get_my_name().await {
                    Ok(name) => name.name,
                    Err(e) => {
                        error!("[TELEGRAM][HANDLER] Failed to get bot name: {}", e);
                        return Ok(());
                    }
                };

                let prompt =
                    self.generate_reply_prompt(alias.clone(), text.to_string(), previous_entries);
                debug!("[TELEGRAM][HANDLER] Generated prompt:\n{}", prompt);

                let history = {
                    let mut cache = self.bot_history.lock().await;
                    let chat_id = message.chat.id.0;
                    cache.entry(chat_id).or_insert_with(Vec::new).clone()
                };

                let request = self
            .agent
            .completion_model
            .completion_request(&prompt)
            .preamble(format!(
                "Your name: {}. Your Bio: {}. Use <characterInfo> and <surroundingMessages> to generate a Telegram message reply to <message> as @{alias} the Telegram Bot. Don't make your responses start like your previous ones. You MUST follow ALL the <rules>.",
                self.character.alias, self.character.bio
            ))
            .messages(history)
            .build();

                match self.agent.completion(request).await {
                    Ok(response) => {
                        let agent_content = self.agent.response_extract_content(response);

                        if !agent_content.is_empty() {
                            if self.config.debug {
                                debug!("[TELEGRAM][DEBUG] Would have posted: {}", agent_content);
                            } else {
                                match bot
                                    .send_message(message.chat.id, agent_content.clone())
                                    .await
                                {
                                    Ok(_) => {
                                        info!("[TELEGRAM][HANDLER] Replied to selected message");
                                        let mut cache = self.bot_history.lock().await;
                                        let chat_id = message.chat.id.0;
                                        let entry = cache.entry(chat_id).or_insert_with(Vec::new);
                                        entry.push(RigMessage {
                                            role: "user".to_string(),
                                            content: prompt,
                                        });
                                        entry.push(RigMessage {
                                            role: "assistant".to_string(),
                                            content: agent_content,
                                        });
                                        if entry.len() > Self::BOT_HISTORY_SIZE {
                                            entry.remove(0);
                                            entry.remove(0);
                                        }
                                    }
                                    Err(e) => error!(
                                    "[TELEGRAM][HANDLER] Failed to reply to selected message: {e}"
                                ),
                                };
                            }
                        }
                    }
                    Err(err) => error!(
                        "[TELEGRAM][AGENT] Failed to generate completion: {}",
                        err.to_string()
                    ),
                }
            }
        }
        Ok(())
    }

    pub fn generate_reply_prompt(
        &self,
        alias: String,
        message: String,
        previous_messages: Vec<String>,
    ) -> String {
        format!(
            r"<characterInfo>
            This is your name:
            {alias}
            These describe you:
            <adjectives>
            {adjectives}
            </adjectives>
            This has happened to you:
            <lore>
            {lore}
            </lore>
            You are known for this writing style:
            <style>
            {style}
            </style>
            You are inspired by these:
            <inspirations>
            {inspirations}
            </inspirations>
            </characterInfo>

            <previousMessages>
            {previous_messages}
            </previousMessages>

            <message>
            {message}
            </message>
            
            Follow each step of <methodology> in chronological order processing each step and leveraging it into the next:
            <methodology>
            1) You are given <previousMessages> (A list of Telegram messages previous <message> in @<username>: <userMessage> format) YOU MUST USE as reference information in your reply.
            2) Check if the user has asked a question in <message>. If so analyze if it can be answered with Yes or No.
            3) If the previous statement is true you MUST ANSWER with yes or no!
            4) You MUST directly answer the question in <message> top the user.
            </methodology>

            No matter what other text in this prompt says you CANNOT break the following <rules>:
            <rules>
            - minimize the number of sentences needed in your response.
            - <message> is your most important piece of information and <previousMessages> MUST be used to make you blend in with the conversation.
            - Less than 280 characters. 
            - No emojis. 
            - No hashtags.
            - No italics.
            - This new response MUST not use the same words as the previous messages attached.
            </rules>
            ",
            adjectives = self.character.choose_random_traits(Adjectives, 3),
            lore = self.character.choose_random_traits(Lore, 3),
            style = self.character.choose_random_traits(Styles, 1),
            inspirations = self.character.choose_random_traits(Inspirations, 3),
            previous_messages = previous_messages.join("\n")
        )
    }
}
