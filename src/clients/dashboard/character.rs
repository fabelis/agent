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
    character_data: Character,
}

#[derive(Clone)]
pub struct CharacterClient<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum> + 'static,
{
    pub agent: Agent<CM>,
}

impl<CM> CharacterClient<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum> + 'static,
{
    pub fn new(agent: Agent<CM>) -> Self {
        Self { agent }
    }

    pub async fn start(self: Self) {
        info!("[CHARACTER][API] Started");

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
                    "/save",
                    web::post().to(
                        |handler: web::Data<Arc<Self>>, body: web::Json<Body>| async move {
                            handler.save_route(body).await
                        },
                    ),
                )
        })
        .bind(("127.0.0.1", 3002))
        .expect("Failed to bind server");

        info!("[CHARACTER][API] Started HTTP server on 127.0.0.1:3002");
        let _ = server.run().await;
    }

    async fn save_route(&self, body: web::Json<Body>) -> HttpResponse {
        let mut character = body.character_data.clone();
        character.path = format!("{}/{}", CHARACTERS_FOLDER, body.path_name);

        if let Err(e) = character.save_to_file() {
            error!("[CHARACTER][API] Failed to save character: {}", e);
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": format!("Failed to save character: {}", e)
            }));
        }

        HttpResponse::Ok().json(serde_json::json!({
            "message": format!("Character saved to {}", character.path)
        }))
    }
}
