use crate::{
    clients::{ApiConfig, StorytellingConfig, TwitterConfig},
    core::CONFIG_PATH,
};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub clients: Clients,
    pub completion_provider: CompletionProvider,

    #[serde(default = "default_embedding_provider")]
    pub embedding_provider: EmbeddingProvider,
    #[serde(default = "default_db")]
    pub db: DatabaseProvider,
}

pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(CONFIG_PATH)?;
    let config: Config = serde_json::from_str(&config_content)?;
    Ok(config)
}

// DBS
#[derive(Deserialize, Debug, Clone)]
pub enum DatabaseProvider {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "mongodb")]
    MongoDB,
}

fn default_db() -> DatabaseProvider {
    DatabaseProvider::Local
}

// EMBEDDING PROVIDERS
#[derive(Deserialize, Debug, Clone)]
pub enum EmbeddingProvider {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "openai")]
    OpenAI,
    #[serde(rename = "xai")]
    XAI,
}

fn default_embedding_provider() -> EmbeddingProvider {
    EmbeddingProvider::Local
}

// PROVIDERS
#[derive(Deserialize, Debug, Clone)]
pub enum CompletionProvider {
    #[serde(rename = "anthropic")]
    Anthropic,
    #[serde(rename = "cohere")]
    Cohere,
    #[serde(rename = "gemini")]
    Gemini,
    #[serde(rename = "openai")]
    OpenAI,
    #[serde(rename = "perplexity")]
    Perplexity,
    #[serde(rename = "xai")]
    XAI,
}

// Clients
#[derive(Deserialize, Debug, Clone)]
pub struct Clients {
    pub api: Option<ApiConfig>,
    pub cli: Option<bool>,
    pub storytelling: Option<StorytellingConfig>,
    pub twitter: Option<TwitterConfig>,
}
