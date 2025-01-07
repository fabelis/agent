use async_trait::async_trait;
use fastembed::{EmbeddingModel as FastEmbedModel, InitOptions, TextEmbedding};
use std::sync::Arc;
use std::{future::Future, pin::Pin};

#[derive(Clone)]
pub struct EmbeddingModel {
    model: Arc<TextEmbedding>,
}

impl EmbeddingModel {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let model = TextEmbedding::try_new(
            InitOptions::new(FastEmbedModel::AllMiniLML6V2).with_show_download_progress(true),
        )?;
        Ok(Self {
            model: Arc::new(model),
        })
    }
}

#[async_trait]
impl rig::embeddings::EmbeddingModel for EmbeddingModel {
    const MAX_DOCUMENTS: usize = 256;

    fn ndims(&self) -> usize {
        384
    }

    #[allow(refining_impl_trait)]
    fn embed_texts(
        &self,
        texts: impl IntoIterator<Item = String> + Send,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        Vec<rig::embeddings::Embedding>,
                        rig::embeddings::EmbeddingError,
                    >,
                > + Send,
        >,
    > {
        let texts: Vec<String> = texts.into_iter().collect();
        let model = self.model.clone();

        Box::pin(async move {
            let embeddings = model.embed(texts, None).map_err(|e| {
                rig::embeddings::EmbeddingError::ProviderError(format!(
                    "Local Embedding error: {}",
                    e
                ))
            })?;
            Ok(embeddings
                .into_iter()
                .map(|e| rig::embeddings::Embedding {
                    document: String::new(),
                    vec: e.into_iter().map(|f| f as f64).collect(),
                })
                .collect())
        })
    }
}
