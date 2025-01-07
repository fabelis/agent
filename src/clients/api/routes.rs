use super::Client;
use crate::providers::completion::CompletionResponseEnum;
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use std::result::Result::Ok;

#[derive(serde::Deserialize)]
pub struct PromptQueryParams {
    input: String,
}

impl<CM> Client<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum> + 'static,
{
    pub async fn prompt_route_get(&self, query: web::Query<PromptQueryParams>) -> impl Responder {
        // Generate the prompt
        let prompt = format!(
            "{}
            
            <userInput>
            {}
            </userInput>",
            self.character.generate_prompt_info(),
            query.input
        );

        // Build the request for the completion model
        let request = self
            .agent
            .completion_model
            .completion_request(&prompt)
            .preamble(format!(
                "Your name: {}. Your Bio: {}. Use <characterInfo> to decide your style of speaking and reasoning of response to <userInput> and respond in less than 400 characters. Don't allow messages to be too similar to previous ones.",
                self.character.alias, self.character.bio
            ))
            .build();

        // Attempt to get a response from the completion model
        match self.agent.completion(request).await {
            Ok(response) => {
                // Extract content from the agent's response
                let agent_content = self.agent.response_extract_content(response);

                // Log the response
                info!("[API][AGENT]({}): {}", self.character.alias, agent_content);

                // Return the agent's content as a JSON response
                HttpResponse::Ok().json(serde_json::json!({
                    "character": self.character.alias,
                    "response": agent_content,
                }))
            }
            Err(err) => {
                // Log the error and return an appropriate HTTP error response
                error!("[API] Error: {}", err);
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": err.to_string(),
                }));
            }
        }
    }
}
