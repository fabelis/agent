use reqwest::{header, Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder};

#[derive(Clone)]
pub struct Client {
    pub http_client: ReqwestClient,
}

impl Client {
    pub const BASE_URL: &str = "https://api.elevenlabs.io";

    pub fn new(api_key: String) -> Result<Self, anyhow::Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert("xi-api-key", header::HeaderValue::try_from(api_key)?);

        let http_client = ReqwestClientBuilder::new()
            .default_headers(headers)
            .build()?;

        Ok(Self { http_client })
    }
}
