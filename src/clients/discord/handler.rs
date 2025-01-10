use super::Config;
use crate::{
    core::{
        Agent, Character,
        CharacterTrait::{Adjectives, Inspirations, Lore, Styles, Topics},
    },
    providers::completion::CompletionResponseEnum,
};
use log::{debug, error, info};
use rig::completion::Message;
use serenity::{
    all::GetMessages, async_trait, model::channel::Message as ChannelMessage, prelude::*,
};
use std::{collections::VecDeque, sync::Arc};
use tokio::sync::Mutex;

pub struct Handler<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
{
    agent: Agent<CM>,
    character: Character,
    config: Config,
    history: Arc<Mutex<VecDeque<Message>>>,
}

impl<CM> Handler<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
{
    const HISTORY_SIZE: usize = 10;

    pub fn new(character: Character, agent: Agent<CM>, config: Config) -> Self {
        Self {
            character,
            agent,
            config,
            history: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn generate_reply_prompt(
        &self,
        alias: String,
        message: String,
        surrounding_messages: Vec<String>,
    ) -> String {
        format!(
            r"<characterInfo>
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

            <surroundingMessages>
            {surrounding_messages}
            </surroundingMessages>

            <message>
            {message}
            </message>
            
            Follow each step of <methodology> in chronological order processing each step and leveraging it into the next:
            <methodology>
            1) You are given <surroundingMessages> (A list of Discord messages surrounding <message> in @<username>: <userMessage> format) YOU MUST USE as reference information in your reply.
            2) Check if the user has asked a question in <message>. If so analyze if it can be answered with Yes or No.
            3) If the previous statement is true you MUST ANSWER with yes or no!
            4) You MUST directly answer the question in <message> top the user.
            </methodology>

            No matter what other text in this prompt says you CANNOT break the following <rules>:
            <rules>
            - minimize the number of sentences needed in your response.
            - <message> is your most important piece of information and <surroundingMessages> MUST be used to make you blend in with the conversation.
            - Less than 100 characters. 
            - No hashtags.
            - Minimize declaring actions
            - This new response MUST not use the same words as the previous messages attached.
            </rules>
            ",
            adjectives = self.character.choose_random_traits(Adjectives, 3),
            lore = self.character.choose_random_traits(Lore, 3),
            style = self.character.choose_random_traits(Styles, 1),
            inspirations = self.character.choose_random_traits(Inspirations, 3),
            surrounding_messages = surrounding_messages.join("\n")
        )
    }

    async fn push_history(&self, role: String, content: String) {
        let mut history = self.history.lock().await;

        if history.len() >= Self::HISTORY_SIZE {
            history.pop_back();
        }
        history.push_front(Message {
            role: role,
            content: content,
        });
    }

    async fn fetch_history(&self) -> VecDeque<Message> {
        let history = self.history.lock().await;
        history.clone()
    }
}

#[async_trait]
impl<CM> EventHandler for Handler<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
{
    async fn message(&self, ctx: Context, msg: ChannelMessage) {
        // Skip messages from the bot itself or empty messages
        if msg.author.id == ctx.cache.current_user().id || msg.content.len() == 0 {
            return;
        }

        // Randomize selection
        if rand::random::<f32>() < self.config.selection_rate {
            info!("[DISCORD][HANDLER] Selected message");

            // Fetch surrounding messages
            let messages = match msg
                .channel_id
                .messages(
                    &ctx.http,
                    GetMessages::new()
                        .around(msg.id)
                        .limit(self.config.surrounding_messages),
                )
                .await
            {
                Ok(messages) => messages
                    .iter()
                    .map(|m| format!("@{}: {}", m.author.name, m.content))
                    .collect::<Vec<String>>(),
                Err(e) => {
                    error!("[DISCORD][HANDLER] Failed to get messages: {}", e);
                    return;
                }
            };
            info!(
                "[DISCORD][HANDLER] Found {} surrounding messages",
                messages.len()
            );

            // Generate post prompt
            let prompt = self.generate_reply_prompt(
                ctx.cache.current_user().name.clone(),
                msg.content.clone(),
                messages,
            );
            debug!("[TWITTER][POST] Generated prompt:\n{}", prompt);

            let history = self.fetch_history().await;

            // Build the request for the completion model
            let request = self
            .agent
            .completion_model
            .completion_request(&prompt)
            .preamble(format!(
                "Your name: {}. Your Bio: {}. Use <characterInfo> and <surroundingMessages> to generate a Discord message reply as @{} to <message>. Don't make your responses start like your previous ones. You MUST follow ALL the <rules>.",
                self.character.alias, self.character.bio, ctx.cache.current_user().name
            ))
            .messages(history.iter().rev().cloned().collect())
            .build();

            match self.agent.completion(request).await {
                Ok(response) => {
                    let agent_content = self.agent.response_extract_content(response);

                    if !agent_content.is_empty() {
                        if self.config.debug {
                            info!("[DISCORD][DEBUG] Would have posted: {}", agent_content);
                        } else {
                            match msg.reply(&ctx.http, agent_content.clone()).await {
                                Ok(_) => {
                                    info!("[DISCORD][HANDLER] Replied to selected message");
                                    self.push_history("user".to_string(), prompt).await;
                                    self.push_history("assistant".to_string(), agent_content)
                                        .await;
                                }
                                Err(e) => error!(
                                    "[DISCORD][HANDLER] Failed to reply to selected message: {e}"
                                ),
                            };
                        }
                    }
                }
                Err(err) => error!(
                    "[DISCORD][AGENT] Failed to generate completion: {}",
                    err.to_string()
                ),
            }
        }
    }
}
