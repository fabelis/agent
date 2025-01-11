use serde::{Deserialize, Serialize};

use super::{Client, OutputFormat};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TtvRequestBody {
    pub text: String,
    pub voice_description: String,
}

#[derive(Default)]
pub struct TtvRequestBuilder {
    text: String,
    voice_description: String,
    output_format: Option<OutputFormat>,
}

impl TtvRequestBuilder {
    pub fn new(text: String, voice_description: String) -> Self {
        Self {
            text,
            voice_description,
            output_format: None,
        }
    }

    pub fn output_format(mut self, output_format: OutputFormat) -> Self {
        self.output_format = Some(output_format);
        self
    }

    pub fn build(self) -> TtvRequest {
        let output_format = self
            .output_format
            .unwrap_or_else(|| OutputFormat::Mp344100192);

        TtvRequest {
            text: self.text,
            voice_description: self.voice_description,
            output_format,
        }
    }
}

pub struct TtvRequest {
    text: String,
    voice_description: String,
    output_format: OutputFormat,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TtvResponse {
    previews: Vec<TtvPresonsePreview>,
    text: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TtvPresonsePreview {
    audio_base_64: String,
    generated_voice_id: String,
    media_type: String,
    duration_secs: f32,
}

impl Client {
    pub async fn ttv(&self, request: TtvRequest) -> Result<Vec<String>, reqwest::Error> {
        let url = format!(
            "{}/v1/text-to-voice/create-previews?output_format={}",
            Self::BASE_URL,
            request.output_format.to_string()
        );

        let body = TtvRequestBody {
            text: request.text,
            voice_description: request.voice_description,
        };

        let response = self.http_client.post(&url).json(&body).send().await?;

        if response.status().is_success() {
            let response_data = response.json::<TtvResponse>().await?;
            Ok(response_data
                .previews
                .into_iter()
                .map(|p| p.generated_voice_id)
                .collect())
        } else {
            Err(response.error_for_status().unwrap_err())
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SaveTtvRequestBody {
    voice_name: String,
    voice_description: String,
    generated_voice_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SaveTtvResponse {
    voice_id: String,
}

impl Client {
    pub async fn save_ttv(
        &self,
        voice_name: String,
        voice_description: String,
        generated_voice_id: String,
    ) -> Result<String, reqwest::Error> {
        let url = format!(
            "{}/v1/text-to-voice/create-voice-from-preview",
            Self::BASE_URL
        );

        // Serialize the request body to JSON
        let body = SaveTtvRequestBody {
            voice_name,
            voice_description,
            generated_voice_id,
        };

        let response = self.http_client.post(&url).json(&body).send().await?;

        if response.status().is_success() {
            let response_body = response.json::<SaveTtvResponse>().await?;
            Ok(response_body.voice_id)
        } else {
            Err(response.error_for_status().unwrap_err())
        }
    }
}
