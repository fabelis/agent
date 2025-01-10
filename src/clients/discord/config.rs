use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(try_from = "ConfigRaw")]
pub struct Config {
    pub surrounding_messages: u8,
    pub selection_rate: f32,
    pub debug: bool,
}

#[derive(Deserialize)]
struct ConfigRaw {
    pub surrounding_messages: u8,
    pub selection_rate: f32,
    #[serde(default)]
    pub debug: bool,
}

impl TryFrom<ConfigRaw> for Config {
    type Error = String;

    fn try_from(raw: ConfigRaw) -> Result<Self, Self::Error> {
        let config = Config {
            surrounding_messages: raw.surrounding_messages,
            selection_rate: raw.selection_rate,
            debug: raw.debug,
        };

        if raw.selection_rate <= 0.0 || raw.selection_rate > 1.0 {
            return Err("selection rate must be within the bounds of 0.0 > x >= 1.0".to_string());
        }

        Ok(config)
    }
}
