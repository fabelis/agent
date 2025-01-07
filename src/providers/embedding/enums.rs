use super::LocalEmbeddingModel;
use rig::{
    embeddings::{Embedding, EmbeddingError, EmbeddingModel as RigEmbeddingModel},
    providers::openai::EmbeddingModel as OpenAiEmbeddingModel,
    providers::xai::embedding::EmbeddingModel as XaiEmbeddingModel,
};

#[derive(Clone)]
pub enum EmbeddingModelEnum {
    Local { model: LocalEmbeddingModel },
    OpenAI { model: OpenAiEmbeddingModel },
    XAI { model: XaiEmbeddingModel },
}

impl RigEmbeddingModel for EmbeddingModelEnum {
    const MAX_DOCUMENTS: usize = 1;

    fn ndims(&self) -> usize {
        match self {
            Self::Local { model } => model.ndims(),
            Self::OpenAI { model } => model.ndims(),
            Self::XAI { model } => model.ndims(),
        }
    }

    async fn embed_texts(
        &self,
        texts: impl IntoIterator<Item = String> + Send,
    ) -> Result<Vec<Embedding>, EmbeddingError> {
        match self {
            Self::Local { model } => model.embed_texts(texts).await,
            Self::OpenAI { model } => model.embed_texts(texts).await,
            Self::XAI { model } => model.embed_texts(texts).await,
        }
    }
}
