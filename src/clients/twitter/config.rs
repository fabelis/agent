use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(try_from = "ConfigRaw")]
pub struct Config {
    pub post_delay: Vec<u8>,
    pub reply_delay: Vec<u8>,
    pub search_delay: u8,
    pub delay: u8,
    pub debug: bool,
}

#[derive(Deserialize)]
struct ConfigRaw {
    post_delay: Vec<u8>,
    reply_delay: Vec<u8>,
    #[serde(default)]
    search_delay: u8,
    #[serde(default)]
    delay: u8,
    #[serde(default)]
    debug: bool,
}

impl TryFrom<ConfigRaw> for Config {
    type Error = String;

    fn try_from(raw: ConfigRaw) -> Result<Self, Self::Error> {
        let config = Config {
            post_delay: raw.post_delay,
            reply_delay: raw.reply_delay,
            search_delay: raw.search_delay,
            delay: raw.delay,
            debug: raw.debug,
        };

        if config.post_delay.len() != 2 {
            return Err("post_delay must have exactly 2 elements".to_string());
        }
        if config.reply_delay.len() != 2 {
            return Err("reply_delay must have exactly 2 elements".to_string());
        }
        if config.post_delay[0] > config.post_delay[1] {
            return Err("post_delay[0] must be <= post_delay[1]".to_string());
        }
        if config.reply_delay[0] > config.reply_delay[1] {
            return Err("reply_delay[0] must be <= reply_delay[1]".to_string());
        }

        Ok(config)
    }
}
