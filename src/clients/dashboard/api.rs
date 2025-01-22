use super::Client;
use super::{CharacterGenBody, ChatPromptBody};
use crate::providers::completion::CompletionResponseEnum;
use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use log::{error, info};
use std::sync::Arc;

impl<CM, EM> Client<CM, EM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum> + 'static,
    EM: rig::embeddings::EmbeddingModel + 'static,
{
    pub async fn start_api(self: Arc<Self>) {
        info!("[DASHBOARD][API] Started");
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
                .app_data(web::Data::new(self.clone()))
                .route(
                    "/chat/prompt",
                    web::post().to(
                        |handler: web::Data<Arc<Self>>, body: web::Json<ChatPromptBody>| async move {
                            handler.chat_prompt_route(body).await
                        },
                    ),
                )
                .route(
                    "/character/gen",         
                            web::post().to(
                    |handler: web::Data<Arc<Self>>, body: web::Json<CharacterGenBody>| async move {
                            handler.character_gen_route(body).await
                        },
                    ),
                )
        })
        .bind(("127.0.0.1", 3001))
        .expect("Failed to bind server");

        info!("[DASHBOARD][API] Started HTTP server on 127.0.0.1:3001");
        if let Err(e) = server.run().await {
            error!("[DASHBOARD][API] Server error: {}", e);
        }
    }
}
