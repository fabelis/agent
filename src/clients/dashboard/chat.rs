use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use log::{error, info};
use rig::completion::Message;
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    core::{Agent, Character, CHARACTERS_FOLDER},
    providers::completion::CompletionResponseEnum,
};

#[derive(Deserialize, Clone)]
struct Body {
    path_name: String,
    prompt: String,
    history: Vec<Message>,
}

#[derive(Clone)]
pub struct ChatClient<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum> + 'static,
{
    pub agent: Agent<CM>,
}

impl<CM> ChatClient<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum> + 'static,
{
    pub fn new(agent: Agent<CM>) -> Self {
        Self { agent }
    }

    pub async fn start(self: Self) {
        info!("[CHAT][API] Started");

        let server = HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .wrap(
                    Cors::default()
                        .allow_any_origin()
                        .allow_any_method()
                        .allow_any_header()
                        .max_age(3600),
                )
                .app_data(web::Data::new(Arc::new(self.clone())))
                .route(
                    "/prompt",
                    web::post().to(
                        |handler: web::Data<Arc<Self>>, body: web::Json<Body>| async move {
                            handler.prompt_route(body).await
                        },
                    ),
                )
        })
        .bind(("127.0.0.1", 3001))
        .expect("Failed to bind server");

        info!("[CHAT][API] Started HTTP server on 127.0.0.1:3001");
        let _ = server.run().await;
    }

    async fn prompt_route(&self, body: web::Json<Body>) -> HttpResponse {
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

        info!("{:?}", body.clone().history);

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
