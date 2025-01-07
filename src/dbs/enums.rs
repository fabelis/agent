use crate::{
    core::MemoryStore,
    dbs::{LocalMemoryStore, MongoDbMemoryStore},
};
use rig::{embeddings::EmbeddingModel, Embed};
use serde::Serialize;

#[derive(Clone)]
pub enum MemoryStoreEnum<D, EM>
where
    D: Embed + Serialize + Send + Sync + Eq + Clone,
    EM: EmbeddingModel,
{
    Local {
        memory_store: LocalMemoryStore<D, EM>,
    },
    MongoDB {
        memory_store: MongoDbMemoryStore<D, EM>,
    },
}

impl<D, EM> MemoryStore<D, EM> for MemoryStoreEnum<D, EM>
where
    D: Embed + Serialize + Send + Sync + Eq + Clone,
    EM: EmbeddingModel,
{
    async fn add(&mut self, document: D) -> Result<(), anyhow::Error> {
        match self {
            Self::Local { memory_store } => memory_store.add(document).await,
            Self::MongoDB { memory_store } => memory_store.add(document).await,
        }
    }

    async fn add_many(&mut self, documents: Vec<D>) -> Result<(), anyhow::Error> {
        match self {
            Self::Local { memory_store } => memory_store.add_many(documents).await,
            Self::MongoDB { memory_store } => memory_store.add_many(documents).await,
        }
    }

    async fn top_n<T: for<'a> serde::Deserialize<'a> + Send>(
        &self,
        query: &str,
        n: usize,
    ) -> Result<Vec<(f64, String, T)>, rig::vector_store::VectorStoreError> {
        match self {
            Self::Local { memory_store } => memory_store.top_n(query, n).await,
            Self::MongoDB { memory_store } => memory_store.top_n(query, n).await,
        }
    }

    async fn top_n_ids(
        &self,
        query: &str,
        n: usize,
    ) -> Result<Vec<(f64, String)>, rig::vector_store::VectorStoreError> {
        match self {
            Self::Local { memory_store } => memory_store.top_n_ids(query, n).await,
            Self::MongoDB { memory_store } => memory_store.top_n_ids(query, n).await,
        }
    }
}
