use super::Config;
use crate::{
    clients::storytelling::{GenQueryParams, TtsBody},
    core::{Agent, Character},
    providers::{completion::CompletionResponseEnum, elevenlabs},
};
use actix_web::{web, App, HttpServer};
use log::info;
use std::{env, sync::Arc};

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
        let server = HttpServer::new(move || {
            let mut app =
                App::new()
                    .app_data(web::Data::new(self.clone())) // Pass the Arc of client to the app
                    .route(
                        "/gen",
                        web::get().to(
                            |handler: web::Data<Arc<Self>>,
                             query: web::Query<GenQueryParams>| async move {
                                handler.gen_route(query).await
                            },
                        ),
                    );
            if self.config.use_tts {
                let api_key = env::var("ELEVENLABS_API_KEY").expect("ELEVENLABS_API_KEY not set");
                let elevenlabs_client = elevenlabs::Client::new(api_key.clone())
                    .expect("Failed to create ElevenLabs client");
                app = app.route(
                    "/tts",
                    web::get().to(
                        move |handler: web::Data<Arc<Self>>, body: web::Json<TtsBody>| {
                            let client = elevenlabs_client.clone();
                            async move { handler.tts_route(body, client).await }
                        },
                    ),
                );
                let elevenlabs_client =
                    elevenlabs::Client::new(api_key).expect("Failed to create ElevenLabs client");
                app = app.route(
                    "/gen-voice",
                    web::get().to(move |handler: web::Data<Arc<Self>>| {
                        let client = elevenlabs_client.clone();
                        async move { handler.gen_voice_route(client).await }
                    }),
                )
            }

            app
        })
        .bind(("127.0.0.1", port)) // Use api_port from config
        .expect("Failed to bind server");

        // start server
        info!("[STORYTELLER] Started http server on 127.0.0.1:{}", port);
        let _ = server.run().await;
    }
}
