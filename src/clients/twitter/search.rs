use super::Client;
use crate::{core::MemoryStore, providers::completion::CompletionResponseEnum};
use log::{error, info};
use rig::Embed;
use serde::{Deserialize, Serialize};

#[derive(Embed, Clone, Deserialize, Debug, Serialize, Eq, PartialEq, Default)]
pub struct TweetDoc {
    pub id: String,
    pub search: String,
    #[embed]
    pub tweet: String,
}

impl<CM, EM> Client<CM, EM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
    EM: rig::embeddings::EmbeddingModel,
{
    pub async fn search(&mut self, topic: &str) {
        info!("[TWITTER][SEARCH] Searching for new tweets...");
        let tweets = match self.client.search(topic.to_string()).await {
            Ok(tweets) => tweets,
            Err(e) => {
                error!("[TWITTER][SEARCH] Failed to search tweets: {}", e);
                return;
            }
        };
        info!("[TWITTER][SEARCH] Found {} new tweets", tweets.len());
        let docs = tweets
            .into_iter()
            .map(|tweet| TweetDoc {
                id: tweet.id.to_string(),
                search: topic.to_string(),
                tweet: tweet.text,
            })
            .collect::<Vec<TweetDoc>>();

        if let Err(e) = self.search_memory.add_many(docs).await {
            error!("[TWITTER][VECDB] Failed to add tweets to memory: {}", e);
            return;
        }
        info!("[TWITTER][VECDB] Added tweets to memory");
    }
}
