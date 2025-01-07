use super::{Config, PromptQueryParams};
use crate::{
    core::{Agent, Character},
    providers::completion::CompletionResponseEnum,
};
use actix_web::{web, App, HttpServer};
use log::info;
use std::sync::Arc;

pub struct Client<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum> + 'static,
{
    pub agent: Agent<CM>,
    pub character: Character,
    pub config: Config,
}

impl<CM> Client<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum> + 'static,
{
    pub fn new(character: Character, completion_model: CM, config: Config) -> Self {
        Client {
            character,
            agent: Agent::new(completion_model),
            config,
        }
    }

    pub async fn start(self: Arc<Self>) {
        info!("[API] Started");

        // fetch api port
        let port = self.config.port;

        // create api server
        let server = HttpServer::new(move || {
            App::new()
                    .app_data(web::Data::new(self.clone())) // Pass the Arc of client to the app
                    .route(
                        "/prompt",
                        web::get().to(
                            |handler: web::Data<Arc<Self>>,
                             query: web::Query<PromptQueryParams>| async move {
                                handler.prompt_route_get(query).await
                            },
                        ),
                    )
        })
        .bind(("127.0.0.1", port)) // Use api_port from config
        .expect("Failed to bind server");

        // start server
        info!("[API] Started http server on 127.0.0.1:{}", port);
        let _ = server.run().await;
    }
}
