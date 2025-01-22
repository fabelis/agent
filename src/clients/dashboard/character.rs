use super::Client;
use crate::{
    core::{Character, CHARACTERS_FOLDER},
    providers::completion::CompletionResponseEnum,
};
use actix_web::{web, HttpResponse};
use log::error;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CharacterGenBody {
    character_data: Character,
    prompt: String,
    field: CharacterGenField,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CharacterGenField {
    Alias,
    Bio,
    Lore,
    Adjectives,
    Styles,
    Topics,
    Inspirations,
}

impl<CM, EM> Client<CM, EM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
    EM: rig::embeddings::EmbeddingModel,
{
    pub async fn character_gen_route(&self, body: web::Json<CharacterGenBody>) -> HttpResponse {
        let body_character = body.character_data.clone();
        let body_prompt = body.prompt.clone();
        let prompt = match body.field {
            CharacterGenField::Alias => Self::gen_alias_prompt(body_character, body_prompt),
            CharacterGenField::Bio => Self::gen_bio_prompt(body_character, body_prompt),
            CharacterGenField::Lore => Self::gen_lore_prompt(body_character, body_prompt),
            CharacterGenField::Adjectives => {
                Self::gen_adjectives_prompt(body_character, body_prompt)
            }
            CharacterGenField::Styles => Self::gen_styles_prompt(body_character, body_prompt),
            CharacterGenField::Topics => Self::gen_topics_prompt(body_character, body_prompt),
            CharacterGenField::Inspirations => {
                Self::gen_inspirations_prompt(body_character, body_prompt)
            }
        };

        let request = self
            .agent
            .completion_model
            .completion_request(&prompt)
            .preamble(format!(
                "Your name: {}. Your Bio: {}. Use <characterInfo> to decide your style of speaking and reasoning of response to <userInput> and respond in less than 400 characters. Don't allow messages to be too similar to previous ones.",
                body.character_data.alias, body.character_data.bio
            ))
            .build();

        match self.agent.completion(request).await {
            Ok(response) => {
                let agent_content = self.agent.response_extract_content(response);
                match body.field {
                    CharacterGenField::Alias | CharacterGenField::Bio => {
                        HttpResponse::Ok().json(serde_json::json!({
                            "content": agent_content
                        }))
                    }
                    _ => HttpResponse::Ok().json(serde_json::json!({
                        "content": agent_content
                            .split('\n')
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>()
                    })),
                }
            }
            Err(e) => {
                error!(
                    "[CHARACTER][API][AGENT] Failed to generate character field: {}",
                    e
                );
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": format!("Failed to generate character field: {}", e)
                }))
            }
        }
    }

    fn gen_alias_prompt(character_data: Character, prompt: String) -> String {
        "Generate an alias for your character".to_string()
    }

    fn gen_bio_prompt(character_data: Character, prompt: String) -> String {
        "Generate a bio for your character".to_string()
    }

    fn gen_lore_prompt(character_data: Character, prompt: String) -> String {
        "Generate a piece of lore for your character".to_string()
    }

    fn gen_adjectives_prompt(character_data: Character, prompt: String) -> String {
        "Generate an adjective that describes your character".to_string()
    }

    fn gen_styles_prompt(character_data: Character, prompt: String) -> String {
        "Generate a style that your character is known for".to_string()
    }

    fn gen_topics_prompt(character_data: Character, prompt: String) -> String {
        "Generate a topic that your character is interested in".to_string()
    }

    fn gen_inspirations_prompt(character_data: Character, prompt: String) -> String {
        "Generate an inspiration for your character".to_string()
    }
}
