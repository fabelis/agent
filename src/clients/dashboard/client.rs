use super::ChatClient;
use crate::{
    clients::dashboard::CharacterClient,
    core::{Agent, Config},
    providers::completion::CompletionResponseEnum,
};
use log::{error, info};

#[derive(Clone)]
pub struct Client<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum> + 'static,
{
    pub agent: Agent<CM>,
    pub config: Config,
}

impl<CM> Client<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
{
    pub fn new(completion_model: CM, config: Config) -> Self {
        Self {
            agent: Agent::new(completion_model),
            config,
        }
    }

    pub async fn start(&mut self) {
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

        let chat_client = ChatClient::new(self.agent.clone());
        let chat_handle = tokio::spawn(async move {
            chat_client.start().await;
        });

        let character_client = CharacterClient::new(self.agent.clone());
        let character_handle = tokio::spawn(async move {
            character_client.start().await;
        });

        let _ = tokio::try_join!(bun_handle, chat_handle, character_handle);
    }
}
