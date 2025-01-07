use crate::core::memory::MemoryStore as CoreMemoryStore;
use mongodb::{bson::Document, options::ClientOptions, Client as MongoClient, Collection};
use rig::{
    embeddings::{Embedding, EmbeddingModel, EmbeddingsBuilder},
    vector_store::{VectorStoreError, VectorStoreIndexDyn},
    Embed, OneOrMany,
};
use rig_mongodb::{MongoDbVectorIndex, SearchParams};
use serde::{Deserialize, Serialize};
use std::future::Future;

#[derive(Clone)]
pub struct MemoryStore<D, EM>
where
    D: Serialize + Send + Sync + Eq + Clone,
    EM: EmbeddingModel,
{
    collection: Collection<Document>,
    embedding_model: EM,
    data_to_doc: fn(data: &D, embedding: &OneOrMany<Embedding>) -> Document,
}

impl<D, EM> MemoryStore<D, EM>
where
    D: Embed + Serialize + Send + Sync + Eq + Clone,
    EM: EmbeddingModel,
{
    pub async fn new(
        conn_url: String,
        db_name: &String,
        coll_name: &String,
        embedding_model: EM,
        data_to_doc: fn(data: &D, embedding: &OneOrMany<Embedding>) -> Document,
    ) -> Result<Self, anyhow::Error> {
        let options = ClientOptions::parse(conn_url)
            .await
            .expect("MongoDB connection string should be valid");

        let mongodb_client =
            MongoClient::with_options(options).expect("MongoDB client options should be valid");

        let collection: Collection<Document> =
            mongodb_client.database(db_name).collection(coll_name);

        Ok(Self {
            collection,
            embedding_model,
            data_to_doc,
        })
    }
}

impl<D, EM> CoreMemoryStore<D, EM> for MemoryStore<D, EM>
where
    D: Embed + Serialize + Send + Sync + Eq + Clone,
    EM: EmbeddingModel,
{
    fn add(&mut self, document: D) -> impl Future<Output = Result<(), anyhow::Error>> + Send {
        async {
            let embedding = EmbeddingsBuilder::new(self.embedding_model.clone())
                .document(document)?
                .build()
                .await?;

            let mongo_documents = embedding
                .iter()
                .map(|(data, embedding)| (self.data_to_doc)(data, embedding))
                .collect::<Vec<_>>();

            self.collection
                .insert_one(mongo_documents.first().unwrap())
                .await?;

            Ok(())
        }
    }

    fn add_many(
        &mut self,
        documents: Vec<D>,
    ) -> impl Future<Output = Result<(), anyhow::Error>> + Send {
        async {
            let embeddings = EmbeddingsBuilder::new(self.embedding_model.clone())
                .documents(documents)?
                .build()
                .await?;

            let mongo_documents = embeddings
                .iter()
                .map(|(data, embedding)| (self.data_to_doc)(data, embedding))
                .collect::<Vec<_>>();

            self.collection.insert_many(mongo_documents).await?;
            Ok(())
        }
    }

    fn top_n<T: for<'a> Deserialize<'a> + Send>(
        &self,
        query: &str,
        n: usize,
    ) -> impl Future<Output = Result<Vec<(f64, String, T)>, VectorStoreError>> + Send {
        let query = query.to_string();
        async move {
            let index = MongoDbVectorIndex::new(
                self.collection.clone(),
                self.embedding_model.clone(),
                "vector_index",
                SearchParams::new(),
            )
            .await?;
            let results = index.top_n(&query, n).await?;
            results
                .into_iter()
                .map(|(score, id, mut value)| {
                    if !std::any::type_name::<T>().contains("String") {
                        if let Some(obj) = value.as_object_mut() {
                            if let Some(id_obj) = obj.get("_id").and_then(|id| id.as_object()) {
                                if let Some(oid) = id_obj.get("$oid").and_then(|oid| oid.as_str()) {
                                    obj.insert(
                                        "_id".to_string(),
                                        serde_json::Value::String(oid.to_string()),
                                    );
                                }
                            }
                        }
                    }
                    let t: T = serde_json::from_value(value)?;
                    Ok((score, id, t))
                })
                .collect()
        }
    }

    fn top_n_ids(
        &self,
        query: &str,
        n: usize,
    ) -> impl Future<Output = Result<Vec<(f64, String)>, VectorStoreError>> + Send {
        let query = query.to_string();
        async move {
            let index = MongoDbVectorIndex::new(
                self.collection.clone(),
                self.embedding_model.clone(),
                "vector_index",
                SearchParams::new(),
            )
            .await?;
            let results = index.top_n_ids(&query, n).await?;
            results
                .into_iter()
                .map(|(score, id)| Ok((score, id)))
                .collect()
        }
    }
}
