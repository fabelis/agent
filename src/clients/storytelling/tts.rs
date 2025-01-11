use super::Client;
use crate::providers::{completion::CompletionResponseEnum, elevenlabs};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use std::result::Result::Ok;

#[derive(serde::Deserialize)]
pub struct Body {
    voice_id: String,
    text: String,
}

impl<CM> Client<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum> + 'static,
{
    pub async fn tts_route(
        &self,
        body: web::Json<Body>,
        elevenlabs_client: elevenlabs::Client,
    ) -> impl Responder {
        let elevenlabs_tts_req =
            elevenlabs::tts::TtsRequestBuilder::new(body.voice_id.clone(), body.text.clone())
                .build();
        let tts_response = match elevenlabs_client.tts(elevenlabs_tts_req).await {
            Ok(tts_response) => tts_response,
            Err(e) => {
                error!(
                    "[STORYTELLER][TTS] Failed to generate audio for text: {} | error: {}",
                    body.text, e
                );
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": e.to_string()
                }));
            }
        };
        info!("[STORYTELLER][TTS] Generated audio for text: {}", body.text);
        HttpResponse::Ok().json(tts_response)
    }
}
