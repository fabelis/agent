use super::Config;
use crate::{
    clients::storytelling::RequestQueryParams,
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
        info!("[STORYTELLER] Started");

        // fetch api port
        let port = self.config.port;

        // create api server
        let server =
            HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(self.clone())) // Pass the Arc of client to the app
                    .route(
                        "/request",
                        web::get().to(
                            |handler: web::Data<Arc<Self>>,
                             query: web::Query<RequestQueryParams>| async move {
                                handler.request_route(query).await
                            },
                        ),
                    )
            })
            .bind(("127.0.0.1", port)) // Use api_port from config
            .expect("Failed to bind server");

        // start server
        info!("[STORYTELLER] Started http server on 127.0.0.1:{}", port);
        let _ = server.run().await;
    }
}
