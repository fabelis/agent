use super::Client;
use crate::{core::MemoryStore, providers::completion::CompletionResponseEnum};
use log::{error, info};
use rig::Embed;
use serde::{Deserialize, Serialize};

#[derive(Embed, Clone, Deserialize, Debug, Serialize, Eq, PartialEq, Default)]
pub struct TruthDoc {
    pub id: String,
    pub search: String,
    #[embed]
    pub truth: String,
}

impl<CM, EM> Client<CM, EM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
    EM: rig::embeddings::EmbeddingModel,
{
    pub async fn search(&mut self, topic: &str) {
        info!("[TRUTH][SEARCH] Searching for new truths...");
        let truths = match self.client.search_truths(topic.to_string(), Some(10)).await {
            Ok(truths) => truths,
            Err(e) => {
                error!("[TRUTH][SEARCH] Failed to search truths: {}", e);
                return;
            }
        };
        info!("[TRUTH][SEARCH] Found {} new truths", truths.len());
        let docs = truths
            .into_iter()
            .map(|truth| TruthDoc {
                id: truth.id.to_string(),
                search: topic.to_string(),
                truth: truth.content,
            })
            .collect::<Vec<TruthDoc>>();

        if let Err(e) = self.search_memory.add_many(docs).await {
            error!("[TRUTH][VECDB] Failed to add truths to memory: {}", e);
            return;
        }
        info!("[TRUTH][VECDB] Added truths to memory");
    }
}
