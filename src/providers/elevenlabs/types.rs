#[derive(serde::Deserialize, serde::Serialize)]
pub struct VoiceSettings {
    pub stability: f32,
    pub similarity_boost: f32,
    pub style: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputFormat {
    Mp32205032,  // mp3 with 22.05kHz sample rate at 32kbps
    Mp34410032,  // mp3 with 44.1kHz sample rate at 32kbps
    Mp34410064,  // mp3 with 44.1kHz sample rate at 64kbps
    Mp34410096,  // mp3 with 44.1kHz sample rate at 96kbps
    Mp344100128, // mp3 with 44.1kHz sample rate at 128kbps (default)
    Mp344100192, // mp3 with 44.1kHz sample rate at 192kbps (requires Creator tier)
    Pcm16000,    // PCM format (S16LE) with 16kHz sample rate
    Pcm22050,    // PCM format (S16LE) with 22.05kHz sample rate
    Pcm24000,    // PCM format (S16LE) with 24kHz sample rate
    Pcm44100,    // PCM format (S16LE) with 44.1kHz sample rate (requires Pro tier)
    Ulaw8000,    // µ-law format (u-law) with 8kHz sample rate
}

impl OutputFormat {
    pub fn to_string(&self) -> &'static str {
        match self {
            OutputFormat::Mp32205032 => "mp3_22050_32",
            OutputFormat::Mp34410032 => "mp3_44100_32",
            OutputFormat::Mp34410064 => "mp3_44100_64",
            OutputFormat::Mp34410096 => "mp3_44100_96",
            OutputFormat::Mp344100128 => "mp3_44100_128",
            OutputFormat::Mp344100192 => "mp3_44100_192",
            OutputFormat::Pcm16000 => "pcm_16000",
            OutputFormat::Pcm22050 => "pcm_22050",
            OutputFormat::Pcm24000 => "pcm_24000",
            OutputFormat::Pcm44100 => "pcm_44100",
            OutputFormat::Ulaw8000 => "ulaw_8000",
        }
    }
}
