mod clients;
mod core;
mod dbs;
mod providers;
use clap::Parser;
use clients::CliClient;
use core::{load_config, Character, CompletionProvider, EmbeddingProvider, CHARACTERS_FOLDER};
use dotenv::dotenv;
use fern::colors::{Color, ColoredLevelConfig};
use log::{error, info};
use providers::{
    completion::CompletionModelEnum,
    embedding::{EmbeddingModelEnum, LocalEmbeddingModel},
};
use std::{env, error::Error, sync::Arc};
use tokio::task::JoinSet;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    character: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // parse args
    let args = Args::parse();

    // init logging
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::Magenta);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    info!("Starting FABELIS.AI Agent...");

    // load config.json
    info!("[SETUP] Loading from config.json...");
    let config = load_config().expect("Failed to load config.json");
    info!("[SETUP] Loaded: {:#?}", config);

    // load .env
    dotenv().ok();
    info!("[SETUP] Loaded .env");

    // load character
    let character_path = format!(
        "{}/{}",
        CHARACTERS_FOLDER,
        args.character
            .unwrap_or("character.example.json".to_string())
    );
    info!("[SETUP] Loading character: {}", character_path);
    let mut character = Character::new(character_path);
    character
        .load()
        .expect("Failed to load character from file");

    // load completion model
    let completion_model: CompletionModelEnum = match config.completion_provider {
        CompletionProvider::Anthropic => {
            let api_key = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not set");
            let model =
                env::var("ANTHROPIC_COMPLETION_MODEL").expect("ANTHROPIC_COMPLETION_MODEL not set");
            let client = rig::providers::anthropic::ClientBuilder::new(&api_key).build();
            let model = CompletionModelEnum::Anthropic(client.completion_model(&model));
            info!("[SETUP] Loaded Anthropic Completion Model");
            model
        }
        CompletionProvider::Cohere => {
            let api_key = env::var("COHERE_API_KEY").expect("COHERE_API_KEY not set");
            let model =
                env::var("COHERE_COMPLETION_MODEL").expect("COHERE_COMPLETION_MODEL not set");
            let client = rig::providers::cohere::Client::new(&api_key);
            let model = CompletionModelEnum::Cohere(client.completion_model(&model));
            info!("[SETUP] Loaded Cohere Completion Model");
            model
        }
        CompletionProvider::Gemini => {
            let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
            let model =
                env::var("GEMINI_COMPLETION_MODEL").expect("GEMINI_COMPLETION_MODEL not set");
            let client = rig::providers::gemini::Client::new(&api_key);
            let model = CompletionModelEnum::Gemini(client.completion_model(&model));
            info!("[SETUP] Loaded Gemini Completion Model");
            model
        }
        CompletionProvider::OpenAI => {
            let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
            let model =
                env::var("OPENAI_COMPLETION_MODEL").expect("OPENAI_COMPLETION_MODEL not set");
            let client = rig::providers::openai::Client::new(&api_key);
            let model = CompletionModelEnum::OpenAI(client.completion_model(&model));
            info!("[SETUP] Loaded OpenAI Completion Model");
            model
        }
        CompletionProvider::Perplexity => {
            let api_key = env::var("PERPLEXITY_API_KEY").expect("PERPLEXITY_API_KEY not set");
            let model = env::var("PERPLEXITY_COMPLETION_MODEL")
                .expect("PERPLEXITY_COMPLETION_MODEL not set");
            let client = rig::providers::perplexity::Client::new(&api_key);
            let model = CompletionModelEnum::Perplexity(client.completion_model(&model));
            info!("[SETUP] Loaded Perplexity Completion Model");
            model
        }
        CompletionProvider::XAI => {
            let api_key = env::var("XAI_API_KEY").expect("XAI_API_KEY not set");
            let model = env::var("XAI_COMPLETION_MODEL").expect("XAI_COMPLETION_MODEL not set");
            let client = rig::providers::xai::Client::new(&api_key);
            let model = CompletionModelEnum::XAI(client.completion_model(&model));
            info!("[SETUP] Loaded XAI Completion Model");
            model
        }
    };

    // load embedding model
    let embedding_model: EmbeddingModelEnum = match config.embedding_provider {
        EmbeddingProvider::Local => {
            let provider =
                LocalEmbeddingModel::new().expect("Failed to initialize local embedding model");
            info!("[SETUP] Loaded Local Embedding Model");
            EmbeddingModelEnum::Local { model: provider }
        }
        EmbeddingProvider::OpenAI => {
            let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
            let client = rig::providers::openai::Client::new(&api_key);
            let provider =
                rig::providers::openai::EmbeddingModel::new(client, "text-embedding-ada-002", 1536);
            info!("[SETUP] Loaded OpenAI Embedding Model");
            EmbeddingModelEnum::OpenAI { model: provider }
        }
        EmbeddingProvider::XAI => {
            let api_key = env::var("XAI_API_KEY").expect("XAI_API_KEY not set");
            let client = rig::providers::xai::Client::new(&api_key);
            let provider =
                rig::providers::xai::embedding::EmbeddingModel::new(client, "text-embedding", 768);
            info!("[SETUP] Loaded XAI Embedding Model");
            EmbeddingModelEnum::XAI { model: provider }
        }
    };

    // cli and other clients cannot run at the same time
    if config.clients.cli.is_some() {
        let mut cli_client = CliClient::new(character, completion_model);
        cli_client.start().await;
    } else {
        // store clients using JoinSet for concurrency
        let mut join_set = JoinSet::new();

        if config.clients.api.is_some() {
            let config = config.clone().clients.api.unwrap();
            let client = Arc::new(clients::ApiClient::new(
                character.clone(),
                completion_model.clone(),
                config,
            ));
            join_set.spawn(async move {
                client.start().await;
            });
        }
        if config.clients.storytelling.is_some() {
            let config = config.clone().clients.storytelling.unwrap();
            let client = Arc::new(clients::StoryTellingClient::new(
                character.clone(),
                completion_model.clone(),
                config,
            ));
            join_set.spawn(async move {
                client.start().await;
            });
        }
        if config.clients.twitter.is_some() {
            let mut client = clients::TwitterClient::new(
                character.clone(),
                completion_model.clone(),
                embedding_model.clone(),
                config.clone().clients.twitter.unwrap(),
                config,
            )
            .await;
            join_set.spawn(async move {
                client.start().await;
            });
        }

        // start clients
        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(_) => {}
                Err(e) => error!("[CLIENTS] Failed: {:?}", e),
            }
        }
    }

    Ok(())
}