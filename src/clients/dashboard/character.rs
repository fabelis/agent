use super::Client;
use crate::{core::Character, providers::completion::CompletionResponseEnum};
use actix_web::{web, HttpResponse};
use log::{error, info};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CharacterGenBody {
    character_data: Character,
    prompt: String,
    field: CharacterGenField,
    keep_current: bool,
    num_fields: usize,
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
            CharacterGenField::Lore => {
                Self::gen_lore_prompt(body_character, body_prompt, body.num_fields)
            }
            CharacterGenField::Adjectives => {
                Self::gen_adjectives_prompt(body_character, body_prompt, body.num_fields)
            }
            CharacterGenField::Styles => {
                Self::gen_styles_prompt(body_character, body_prompt, body.num_fields)
            }
            CharacterGenField::Topics => {
                Self::gen_topics_prompt(body_character, body_prompt, body.num_fields)
            }
            CharacterGenField::Inspirations => {
                Self::gen_inspirations_prompt(body_character, body_prompt, body.num_fields)
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
                info!(
                    "[CHARACTER][API][AGENT] Generated content: {}",
                    agent_content
                );
                match body.field {
                    CharacterGenField::Alias | CharacterGenField::Bio => {
                        HttpResponse::Ok().json(serde_json::json!({
                            "content": agent_content
                        }))
                    }
                    _ => {
                        info!("[CHARACTER][API][AGENT] Handling array field");
                        let mut content = if body.keep_current {
                            match body.field {
                                CharacterGenField::Lore => body.character_data.lore.clone(),
                                CharacterGenField::Adjectives => {
                                    body.character_data.adjectives.clone()
                                }
                                CharacterGenField::Styles => body.character_data.styles.clone(),
                                CharacterGenField::Topics => body.character_data.topics.clone(),
                                CharacterGenField::Inspirations => {
                                    body.character_data.inspirations.clone()
                                }
                                _ => Vec::new(),
                            }
                        } else {
                            Vec::new()
                        };

                        content.extend(
                            agent_content
                                .split('\n')
                                .filter(|s| !s.trim().is_empty())
                                .map(|s| s.to_string()),
                        );

                        HttpResponse::Ok().json(serde_json::json!({
                            "content": content
                        }))
                    }
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
        format!(
            "Follow each step of <methodology> in chronological order:
            <methodology>
            1) Use the character data provided to iterate upon
            2) Follow the user input as guidance
            3) Generate a single memorable alias name
            </methodology>

            <character_data>
            {}
            </character_data>

            <input>
            {}
            </input>

            <rules>
            - Return a single alias name as plain text
            - The alias should be memorable and fitting for the character
            - Do not include any prefixes or suffixes
            </rules>",
            serde_json::to_string_pretty(&character_data).unwrap(),
            prompt
        )
    }

    fn gen_bio_prompt(character_data: Character, prompt: String) -> String {
        format!(
            "Follow each step of <methodology> in chronological order:
            <methodology>
            1) Use the character data provided to iterate upon
            2) Follow the user input as guidance
            3) Generate a compelling character biography
            </methodology>

            <character_data>
            {}
            </character_data>

            <input>
            {}
            </input>

            <rules>
            - Return a single biography as plain text
            - Keep the biography concise but descriptive
            - Do not include any prefixes or suffixes
            </rules>",
            serde_json::to_string_pretty(&character_data).unwrap(),
            prompt
        )
    }

    fn gen_lore_prompt(character_data: Character, prompt: String, num_fields: usize) -> String {
        format!(
            "Follow each step of <methodology> in chronological order:
            <methodology>
            1) Use the character data provided to iterate upon
            2) Follow the user input as guidance
            3) Generate {} pieces of character lore
            </methodology>

            <character_data>
            {}
            </character_data>

            <input>
            {}
            </input>

            <rules>
            - Return exactly {} lore entries separated by newlines
            - Each entry should reveal interesting background details
            - Do not include any prefixes or suffixes
            </rules>",
            num_fields,
            serde_json::to_string_pretty(&character_data).unwrap(),
            prompt,
            num_fields
        )
    }

    fn gen_adjectives_prompt(
        character_data: Character,
        prompt: String,
        num_fields: usize,
    ) -> String {
        format!(
            "Follow each step of <methodology> in chronological order:
            <methodology>
            1) Use the character data provided to iterate upon
            2) Follow the user input as guidance
            3) Generate {} descriptive adjectives
            </methodology>

            <character_data>
            {}
            </character_data>

            <input>
            {}
            </input>

            <rules>
            - Return exactly {} adjectives separated by newlines
            - Each adjective should meaningfully describe the character
            - Do not include any prefixes or suffixes
            </rules>",
            num_fields,
            serde_json::to_string_pretty(&character_data).unwrap(),
            prompt,
            num_fields
        )
    }

    fn gen_styles_prompt(character_data: Character, prompt: String, num_fields: usize) -> String {
        format!(
            "Follow each step of <methodology> in chronological order:
            <methodology>
            1) Use the character data provided to iterate upon
            2) Follow the user input as guidance
            3) Generate {} visual/aesthetic styles
            </methodology>

            <character_data>
            {}
            </character_data>

            <input>
            {}
            </input>

            <rules>
            - Return exactly {} styles separated by newlines
            - Each style should define the character's visual identity
            - Do not include any prefixes or suffixes
            </rules>",
            num_fields,
            serde_json::to_string_pretty(&character_data).unwrap(),
            prompt,
            num_fields
        )
    }

    fn gen_topics_prompt(character_data: Character, prompt: String, num_fields: usize) -> String {
        format!(
            "Follow each step of <methodology> in chronological order:
            <methodology>
            1) Use the character data provided to iterate upon
            2) Follow the user input as guidance
            3) Generate {} topics of interest
            </methodology>

            <character_data>
            {}
            </character_data>

            <input>
            {}
            </input>

            <rules>
            - Return exactly {} topics separated by newlines
            - Each topic should be something the character is knowledgeable about
            - Do not include any prefixes or suffixes
            </rules>",
            num_fields,
            serde_json::to_string_pretty(&character_data).unwrap(),
            prompt,
            num_fields
        )
    }

    fn gen_inspirations_prompt(
        character_data: Character,
        prompt: String,
        num_fields: usize,
    ) -> String {
        format!(
            "Follow each step of <methodology> in chronological order:
            <methodology>
            1) Use the character data provided to iterate upon
            2) Follow the user input as guidance
            3) Generate {} creative inspirations
            </methodology>

            <character_data>
            {}
            </character_data>

            <input>
            {}
            </input>

            <rules>
            - Return exactly {} inspirations separated by newlines
            - Each inspiration should influence the character's design/personality
            - Do not include any prefixes or suffixes
            </rules>",
            num_fields,
            serde_json::to_string_pretty(&character_data).unwrap(),
            prompt,
            num_fields
        )
    }
}
