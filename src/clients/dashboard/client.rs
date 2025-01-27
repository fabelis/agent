use crate::{
    core::{Agent, Config},
    providers::completion::CompletionResponseEnum,
};
use log::{error, info};
use std::sync::Arc;

#[derive(Clone)]
pub struct Client<CM, EM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum> + 'static,
    EM: rig::embeddings::EmbeddingModel + 'static,
{
    pub agent: Agent<CM>,
    pub config: Config,
    pub embedding_model: EM,
}

impl<CM, EM> Client<CM, EM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
    EM: rig::embeddings::EmbeddingModel,
{
    pub fn new(completion_model: CM, embedding_model: EM, config: Config) -> Self {
        Self {
            agent: Agent::new(completion_model),
            config,
            embedding_model,
        }
    }

    pub async fn start(&self) {
        info!("[DASHBAORD] Starting...");
        let dashboard_path = match std::env::current_exe() {
            Ok(exe_path) => match exe_path.parent() {
                Some(parent) => parent.join("../../dashboard"),
                None => {
                    error!("[DASHBAORD] Could not get parent directory of executable");
                    return;
                }
            },
            Err(e) => {
                error!("[DASHBAORD] Failed to get executable path: {}", e);
                return;
            }
        };

        info!("[DASHBAORD] Dashboard path: {:?}", dashboard_path);

        let bun_handle = tokio::spawn(async move {
            if let Err(e) = std::process::Command::new("bun")
                .arg("run")
                .arg("dev")
                .current_dir(&dashboard_path)
                .status()
            {
                error!("[DASHBAORD] Failed to start dashboard frontend: {}", e);
                return;
            }
        });

        let api_handle = tokio::spawn(Arc::new(self.clone()).start_api());

        let _ = tokio::try_join!(bun_handle, api_handle);
    }
}
