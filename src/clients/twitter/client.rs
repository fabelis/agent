use std::collections::VecDeque;

use super::Config;
use super::{search::TweetDoc, TwitterClient};
use crate::core::DatabaseProvider;
use crate::dbs::{LocalMemoryStore, MemoryStoreEnum, MongoDbMemoryStore};
use crate::{
    core::{Agent, Character, Config as RootConfig},
    providers::completion::CompletionResponseEnum,
};
use log::info;
use mongodb::bson;
use rand::Rng;
use rig::{completion::Message, embeddings::Embedding, OneOrMany};
use tokio::{sync::mpsc, time::sleep};
use twitter_v2::id::NumericId;

#[derive(Clone)]
pub struct Client<CM, EM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
    EM: rig::embeddings::EmbeddingModel,
{
    pub agent: Agent<CM>,
    pub character: Character,
    pub config: Config,
    pub client: TwitterClient,
    pub search_memory: MemoryStoreEnum<TweetDoc, EM>,
    pub post_history: VecDeque<Message>,
    pub latest_mention_id: NumericId,
}

enum Action {
    Post(),
    Reply(),
}

impl<CM, EM> Client<CM, EM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
    EM: rig::embeddings::EmbeddingModel,
{
    pub const POST_HISTORY_SIZE: usize = 6;

    pub async fn new(
        character: Character,
        completion_model: CM,
        embedding_model: EM,
        config: Config,
        root_config: RootConfig,
    ) -> Self {
        let client = TwitterClient::new().await;

        let search_memory: MemoryStoreEnum<TweetDoc, EM> = match root_config.db {
            DatabaseProvider::Local {} => MemoryStoreEnum::Local {
                memory_store: LocalMemoryStore::new(embedding_model.clone()),
            },
            DatabaseProvider::MongoDB {} => MemoryStoreEnum::MongoDB {
                memory_store: MongoDbMemoryStore::new(
                    std::env::var("MONGODB_CONN_URL").expect("MONGODB_CONN_URL must be set"),
                    &std::env::var("MONGODB_DB").expect("MONGODB_DB must be set"),
                    &std::env::var("MONGODB_COLLECTION").expect("MONGODB_COLLECTION must be set"),
                    embedding_model.clone(),
                    |data: &TweetDoc, embedding: &OneOrMany<Embedding>| {
                        let mut doc = bson::Document::new();
                        doc.insert("id", data.id.clone());
                        doc.insert("search", data.search.clone());
                        doc.insert("tweet", data.tweet.clone());
                        doc.insert("embedding", embedding.first().vec.clone());
                        doc
                    },
                )
                .await
                .expect("Failed to create MongoDB memory store"),
            },
        };

        Client {
            character,
            agent: Agent::new(completion_model),
            config,
            client,
            search_memory,
            post_history: VecDeque::with_capacity(Self::POST_HISTORY_SIZE),
            latest_mention_id: NumericId::new(0),
        }
    }

    pub async fn start(&mut self) {
        info!("[TWITTER] Starting client with 15s delay...");
        let (sender, mut receiver) = mpsc::channel(3);

        let post_sender = sender.clone();
        let post_config = self.config.clone();
        tokio::spawn(async move {
            sleep(std::time::Duration::from_secs(15)).await;
            loop {
                if post_sender.send(Action::Post()).await.is_err() {
                    break;
                }

                let delay = rand::thread_rng()
                    .gen_range(post_config.post_delay[0]..=post_config.post_delay[1]);
                info!("[TWITTER][POST] sleeping for {} minutes", delay);
                sleep(std::time::Duration::from_secs(u64::from(delay) * 60)).await;
            }
        });

        let reply_sender = sender.clone();
        let reply_config = self.config.clone();
        tokio::spawn(async move {
            sleep(std::time::Duration::from_secs(1)).await;
            loop {
                if reply_sender.send(Action::Reply()).await.is_err() {
                    break;
                }

                let delay = rand::thread_rng()
                    .gen_range(reply_config.reply_delay[0]..=reply_config.reply_delay[1]);
                info!("[TWITTER][REPLY] sleeping for {} minutes", delay);
                sleep(std::time::Duration::from_secs(u64::from(delay) * 60)).await;
            }
        });

        while let Some(action) = receiver.recv().await {
            match action {
                Action::Post() => {
                    info!("[TWITTER][POST] Executing...");
                    let topic = self
                        .character
                        .choose_random_traits(crate::core::CharacterTrait::Topics, 1);

                    self.search(&topic).await;

                    sleep(std::time::Duration::from_secs(
                        u64::from(self.config.search_delay) * 60,
                    ))
                    .await;

                    self.post(topic).await;
                }
                Action::Reply() => {
                    info!("[TWITTER][REPLY] Executing...");
                    self.reply().await;
                }
            }
            sleep(std::time::Duration::from_secs(u64::from(self.config.delay))).await;
        }
    }
}
