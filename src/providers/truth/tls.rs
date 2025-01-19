use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct TlsResponse {
    pub headers: Vec<String>,
    pub cookies: HashMap<String, String>,
    pub body: String,
}

pub fn parse_tls_response(response: &str) -> Result<TlsResponse, anyhow::Error> {
    let parts: Vec<&str> = response.split("\r\n\r\n").collect();
    let headers_section = parts[0];
    let body = parts.get(1).unwrap_or(&"");

    let mut headers = Vec::new();
    let mut cookies = HashMap::new();

    // Parse headers
    for line in headers_section.lines() {
        if line.contains(":") {
            headers.push(line.to_string());

            if line.to_lowercase().starts_with("set-cookie:") {
                let cookie_content = line.splitn(2, ":").nth(1).unwrap_or("").trim();
                if let Some((key, value)) = cookie_content.split(';').next().and_then(|kv| {
                    let parts: Vec<&str> = kv.split('=').collect();
                    if parts.len() == 2 {
                        Some((parts[0].trim(), parts[1].trim()))
                    } else {
                        None
                    }
                }) {
                    cookies.insert(key.to_string(), value.to_string());
                }
            }
        }
    }

    let decoded_body = decode_chunked_body(body)?;

    Ok(TlsResponse {
        headers,
        cookies,
        body: decoded_body,
    })
}

pub fn decode_chunked_body(body: &str) -> Result<String, anyhow::Error> {
    let parts: Vec<&str> = body.split("\r\n").collect();

    if parts.len() > 2 {
        let content = parts[1];
        Ok(content.to_string())
    } else {
        Err(anyhow::anyhow!("Invalid chunked body format"))
    }
}
