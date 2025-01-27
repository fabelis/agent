use actix_web::{web, HttpResponse};
use log::{error, info};
use rig::completion::Message;
use serde::Deserialize;

use super::Client;
use crate::{
    core::{Character, CHARACTERS_FOLDER},
    providers::completion::CompletionResponseEnum,
};

#[derive(Deserialize, Clone)]
pub struct ChatPromptBody {
    path_name: String,
    prompt: String,
    history: Vec<Message>,
}

impl<CM, EM> Client<CM, EM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum> + 'static,
    EM: rig::embeddings::EmbeddingModel + 'static,
{
    pub async fn chat_prompt_route(&self, body: web::Json<ChatPromptBody>) -> HttpResponse {
        let character_path = format!("{}/{}", CHARACTERS_FOLDER, body.clone().path_name);
        let mut character = Character::new(character_path);
        if let Err(e) = character.load() {
            error!("[CHAT][API] Failed to load character: {}", e);
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": format!("Failed to load character: {}", e)
            }));
        }

        let prompt = format!(
            "{}
            
            <userInput>
            {}
            </userInput>",
            character.generate_prompt_info(),
            body.prompt
        );

        let request = self
            .agent
            .completion_model
            .completion_request(&prompt)
            .preamble(format!(
                "Your name: {}. Your Bio: {}. Use <characterInfo> to decide your style of speaking and reasoning of response to <userInput> and respond in less than 400 characters. Don't allow messages to be too similar to previous ones.",
                character.alias, character.bio
            ))
            .messages(body.clone().history)
            .build();

        match self.agent.completion(request).await {
            Ok(response) => {
                let agent_content = self.agent.response_extract_content(response);
                info!("[CHAT][API][AGENT]({}): {}", character.alias, agent_content);

                HttpResponse::Ok().json(serde_json::json!({
                    "response": agent_content,
                }))
            }
            Err(err) => {
                error!("[CHAT][API] Error: {}", err);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": err.to_string(),
                }))
            }
        }
    }
}
