use rig::{
    completion::{CompletionError, CompletionModel, CompletionRequest, CompletionResponse},
    providers::{
        anthropic::completion as anthropic_completion,
        cohere::{self as cohere_completion},
        gemini::completion as gemini_completion,
        openai::{self as openai_completion},
        perplexity::{self as perplexity_completion},
        xai::{self as xai_completion},
    },
};

#[derive(Clone)]
pub enum CompletionModelEnum {
    Anthropic(anthropic_completion::CompletionModel),
    Cohere(cohere_completion::CompletionModel),
    Gemini(gemini_completion::CompletionModel),
    OpenAI(openai_completion::CompletionModel),
    Perplexity(perplexity_completion::CompletionModel),
    XAI(xai_completion::completion::CompletionModel),
}

pub enum CompletionResponseEnum {
    Anthropic(anthropic_completion::CompletionResponse),
    Cohere(cohere_completion::CompletionResponse),
    Gemini(gemini_completion::gemini_api_types::GenerateContentResponse),
    OpenAI(openai_completion::CompletionResponse),
    Perplexity(perplexity_completion::CompletionResponse),
    XAI(xai_completion::completion::xai_api_types::CompletionResponse),
}

impl CompletionModel for CompletionModelEnum {
    type Response = CompletionResponseEnum;

    async fn completion(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse<Self::Response>, CompletionError> {
        match self {
            Self::Anthropic(model) => {
                let response = model.completion(request).await?;
                Ok(CompletionResponse {
                    choice: response.choice,
                    raw_response: CompletionResponseEnum::Anthropic(response.raw_response),
                })
            }
            Self::Cohere(model) => {
                let response = model.completion(request).await?;
                Ok(CompletionResponse {
                    choice: response.choice,
                    raw_response: CompletionResponseEnum::Cohere(response.raw_response),
                })
            }
            Self::Gemini(model) => {
                let response = model.completion(request).await?;
                Ok(CompletionResponse {
                    choice: response.choice,
                    raw_response: CompletionResponseEnum::Gemini(response.raw_response),
                })
            }
            Self::OpenAI(model) => {
                let response = model.completion(request).await?;
                Ok(CompletionResponse {
                    choice: response.choice,
                    raw_response: CompletionResponseEnum::OpenAI(response.raw_response),
                })
            }
            Self::Perplexity(model) => {
                let response = model.completion(request).await?;
                Ok(CompletionResponse {
                    choice: response.choice,
                    raw_response: CompletionResponseEnum::Perplexity(response.raw_response),
                })
            }
            Self::XAI(model) => {
                let response = model.completion(request).await?;
                Ok(CompletionResponse {
                    choice: response.choice,
                    raw_response: CompletionResponseEnum::XAI(response.raw_response),
                })
            }
        }
    }
}
