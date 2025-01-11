use serde::{Deserialize, Serialize};

use super::Client;

#[derive(Deserialize, Serialize)]
pub struct VoiceSettings {
    pub stability: f32,
    pub similarity_boost: f32,
    pub style: i32,
}

#[derive(Deserialize, Serialize)]
pub struct TtsRequestBody {
    pub text: String,
    pub voice_settings: VoiceSettings,
    pub model_id: String,
}

#[derive(Default)]
pub struct TtsRequestBuilder {
    text: String,
    voice_id: String,
    voice_settings: Option<VoiceSettings>,
    model_id: Option<String>,
}

impl TtsRequestBuilder {
    pub fn new(voice_id: String, text: String) -> Self {
        Self {
            text,
            voice_id,
            voice_settings: None,
            model_id: None,
        }
    }

    pub fn voice_settings(mut self, voice_settings: VoiceSettings) -> Self {
        self.voice_settings = Some(voice_settings);
        self
    }

    pub fn model_id(mut self, model_id: String) -> Self {
        self.model_id = Some(model_id);
        self
    }

    pub fn build(self) -> TtsRequest {
        let voice_settings = self.voice_settings.unwrap_or(VoiceSettings {
            stability: 0.5,
            similarity_boost: 0.75,
            style: 0,
        });
        let model_id = self
            .model_id
            .unwrap_or_else(|| "eleven_flash_v2_5".to_string());

        TtsRequest {
            text: self.text,
            voice_settings,
            model_id,
            voice_id: self.voice_id,
        }
    }
}

pub struct TtsRequest {
    pub text: String,
    pub voice_settings: VoiceSettings,
    pub model_id: String,
    pub voice_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TtsResponse {
    pub audio_base64: String,
    pub alignment: Alignment,
    pub normalized_alignment: NormalizedAlignment,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Alignment {
    pub characters: Vec<String>,
    pub character_start_times_seconds: Vec<f64>,
    pub character_end_times_seconds: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NormalizedAlignment {
    pub characters: Vec<String>,
    pub character_start_times_seconds: Vec<f64>,
    pub character_end_times_seconds: Vec<f64>,
}

impl Client {
    pub async fn tts(&self, request: TtsRequest) -> Result<TtsResponse, reqwest::Error> {
        let url = format!(
            "{}/v1/text-to-speech/{}/with-timestamps?optimize_streaming_latency=0&output_format=mp3_22050_32",
            Self::BASE_URL,
            request.voice_id
        );

        let body = TtsRequestBody {
            text: request.text,
            voice_settings: request.voice_settings,
            model_id: request.model_id,
        };

        let response = self.http_client.post(&url).json(&body).send().await?;

        if response.status().is_success() {
            let response_body: TtsResponse = response.json().await?;
            Ok(response_body)
        } else {
            Err(response.error_for_status().unwrap_err())
        }
    }
}
