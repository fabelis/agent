use crate::providers::completion::CompletionResponseEnum;
use rig::completion::{CompletionError, CompletionRequest, CompletionResponse, Message};
use rig::providers::anthropic::completion::Content as AnthropicCompletionContent;

#[derive(Clone)]
pub struct Agent<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
{
    pub completion_model: CM,
}

impl<CM> Agent<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
{
    pub fn new(completion_model: CM) -> Self {
        Self { completion_model }
    }

    pub async fn prompt(&self, prompt: &str) -> Result<String, CompletionError> {
        let request = self.completion_model.completion_request(prompt).build();

        let response = self.completion_model.completion(request).await.unwrap();
        let content = self.response_extract_content(response);
        Ok(content)
    }

    pub async fn chat(
        &self,
        prompt: &str,
        history: Vec<Message>,
    ) -> Result<String, CompletionError> {
        let request = self
            .completion_model
            .completion_request(prompt)
            .messages(history)
            .build();

        let response = self.completion_model.completion(request).await.unwrap();
        let content = self.response_extract_content(response);
        Ok(content)
    }

    pub async fn completion(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse<CompletionResponseEnum>, CompletionError> {
        Ok(self.completion_model.completion(request).await.unwrap())
    }

    pub fn response_extract_content(
        &self,
        response: CompletionResponse<CompletionResponseEnum>,
    ) -> String {
        match response.raw_response {
            CompletionResponseEnum::Anthropic(response) => match &response.content[0] {
                AnthropicCompletionContent::String(text) => text.clone(),
                AnthropicCompletionContent::Text { text, .. } => text.clone(),
                AnthropicCompletionContent::ToolUse { .. } => "Tool use response".to_string(),
            },
            CompletionResponseEnum::Cohere(response) => response.text,
            CompletionResponseEnum::Gemini(response) => response.candidates[0].content.parts[0]
                .text
                .as_ref()
                .expect("Failed to parse Gemini response")
                .to_string(),
            CompletionResponseEnum::OpenAI(response) => response.choices[0]
                .message
                .content
                .as_ref()
                .expect("Failed to parse OpenAI response")
                .to_string(),
            CompletionResponseEnum::Perplexity(response) => {
                response.choices[0].message.content.clone()
            }
            CompletionResponseEnum::XAI(response) => response.choices[0]
                .message
                .content
                .clone()
                .unwrap_or_default(),
        }
    }
}
