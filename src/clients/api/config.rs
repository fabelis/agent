use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub port: u16,
}
