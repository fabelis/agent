use crate::clients::{
    ApiConfig, DiscordConfig, StorytellingConfig, TelegramConfig, TruthConfig, TwitterConfig,
};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub client_configs: ClientConfigs,
    pub enabled_clients: Vec<Clients>,
    pub completion_provider: CompletionProvider,
    #[serde(default = "default_embedding_provider")]
    pub embedding_provider: EmbeddingProvider,
    #[serde(default = "default_db")]
    pub db: DatabaseProvider,
}

impl Config {
    pub fn new(path: String) -> Result<Self, anyhow::Error> {
        let config_content = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&config_content)?;
        Ok(config)
    }
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
    #[serde(rename = "deepseek")]
    DeepSeek,
}

// Client
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum Clients {
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "cli")]
    Cli,
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "storytelling")]
    Storytelling,
    #[serde(rename = "telegram")]
    Telegram,
    #[serde(rename = "twitter")]
    Twitter,
    #[serde(rename = "truth")]
    Truth,
}

// Client Configs
#[derive(Deserialize, Debug, Clone)]
pub struct ClientConfigs {
    pub api: Option<ApiConfig>,
    pub cli: Option<bool>,
    pub discord: Option<DiscordConfig>,
    pub storytelling: Option<StorytellingConfig>,
    pub telegram: Option<TelegramConfig>,
    pub twitter: Option<TwitterConfig>,
    pub truth: Option<TruthConfig>,
}
