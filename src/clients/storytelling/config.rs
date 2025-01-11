use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub paragraph_count: Vec<u8>,
    #[serde(default)]
    pub use_tts: bool,
}
