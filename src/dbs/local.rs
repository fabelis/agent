use crate::core::memory::MemoryStore as CoreMemoryStore;
use rig::{
    embeddings::{EmbeddingModel, EmbeddingsBuilder},
    vector_store::{in_memory_store::InMemoryVectorStore, VectorStoreError, VectorStoreIndexDyn},
    Embed,
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct MemoryStore<D, EM>
where
    D: Serialize + Send + Sync + Eq + Clone,
    EM: EmbeddingModel,
{
    store: InMemoryVectorStore<D>,
    embedding_model: EM,
}

impl<D, EM> MemoryStore<D, EM>
where
    D: Embed + Serialize + Send + Sync + Eq + Clone,
    EM: EmbeddingModel,
{
    pub fn new(embedding_model: EM) -> Self {
        Self {
            store: InMemoryVectorStore::from_documents(vec![]),
            embedding_model,
        }
    }
}

impl<D, EM> CoreMemoryStore<D, EM> for MemoryStore<D, EM>
where
    D: Embed + Serialize + Send + Sync + Eq + Clone,
    EM: EmbeddingModel,
{
    async fn add(&mut self, document: D) -> Result<(), anyhow::Error> {
        let embedding = EmbeddingsBuilder::new(self.embedding_model.clone())
            .document(document)?
            .build()
            .await?;

        self.store.add_documents(embedding);
        Ok(())
    }

    async fn add_many(&mut self, documents: Vec<D>) -> Result<(), anyhow::Error> {
        let embedding = EmbeddingsBuilder::new(self.embedding_model.clone())
            .documents(documents)?
            .build()
            .await?;

        self.store.add_documents(embedding);

        Ok(())
    }

    async fn top_n<T: for<'a> Deserialize<'a> + Send>(
        &self,
        query: &str,
        n: usize,
    ) -> Result<Vec<(f64, String, T)>, VectorStoreError> {
        let query = query.to_string();

        let index = self.store.clone().index(self.embedding_model.clone());
        let results = index.top_n(&query, n).await?;
        results
            .into_iter()
            .map(|(score, id, value)| {
                let t = serde_json::from_value(value)?;
                Ok((score, id, t))
            })
            .collect()
    }

    async fn top_n_ids(
        &self,
        query: &str,
        n: usize,
    ) -> Result<Vec<(f64, String)>, VectorStoreError> {
        let query = query.to_string();

        let index = self.store.clone().index(self.embedding_model.clone());
        let results = index.top_n_ids(&query, n).await?;
        Ok(results.into_iter().collect::<Vec<_>>())
    }

    async fn clear(&mut self) -> Result<(), anyhow::Error> {
        *self = Self::new(self.embedding_model.clone());
        Ok(())
    }

    async fn count(&self) -> Result<usize, anyhow::Error> {
        Ok(self.store.len())
    }
}
