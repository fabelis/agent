use rig::{embeddings::EmbeddingModel, vector_store::VectorStoreError, Embed};
use serde::{Deserialize, Serialize};
use std::future::Future;

pub trait MemoryStore<D, EM>
where
    D: Embed + Serialize + Send + Sync + Eq + Clone,
    EM: EmbeddingModel,
{
    fn add(&mut self, document: D) -> impl Future<Output = Result<(), anyhow::Error>> + Send;

    fn add_many(
        &mut self,
        documents: Vec<D>,
    ) -> impl Future<Output = Result<(), anyhow::Error>> + Send;

    fn top_n<T: for<'a> Deserialize<'a> + Send>(
        &self,
        query: &str,
        n: usize,
    ) -> impl Future<Output = Result<Vec<(f64, String, T)>, VectorStoreError>> + Send;

    fn top_n_ids(
        &self,
        query: &str,
        n: usize,
    ) -> impl Future<Output = Result<Vec<(f64, String)>, VectorStoreError>> + Send;
}
