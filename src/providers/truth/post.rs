use super::client::Client;
use std::io::{Read, Write};

impl Client {
    pub async fn post(&mut self, content: String) -> Result<(), anyhow::Error> {
        let mut stream = self.clone().create_tls_stream()?;

        let body = format!(
            r#"{{"content_type":"text/plain","in_reply_to_id":"","media_ids":[],"poll":null,"quote_id":"","status":"{}","to":[],"visibility":"public","group_timeline_visible":false}}"#,
            content
        );

        let request = format!(
            "POST /api/v1/statuses HTTP/1.1\r\n\
            Host: truthsocial.com\r\n\
            Accept: application/json, text/plain, */*\r\n\
            Accept-Language: en-US,en;q=0.9\r\n\
            Authorization: Bearer {}\r\n\
            Cookie: {}\r\n\
            Content-Type: application/json\r\n\
            Origin: https://truthsocial.com\r\n\
            Referer: https://truthsocial.com/\r\n\
            Sec-CH-UA: \"Google Chrome\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\"\r\n\
            Sec-CH-UA-Mobile: ?0\r\n\
            Sec-CH-UA-Platform: \"Windows\"\r\n\
            Sec-Fetch-Dest: empty\r\n\
            Sec-Fetch-Mode: cors\r\n\
            Sec-Fetch-Site: same-origin\r\n\
            User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36\r\n\
            Content-Length: {}\r\n\
            Connection: close\r\n\
            \r\n\
            {}",
            self.access_token,
            self.cookies.format(),
            body.len(),
            body
        );

        stream.write_all(request.as_bytes())?;

        let mut response = Vec::new();
        stream.read_to_end(&mut response)?;

        let response_str = String::from_utf8(response)?;

        let parsed_response = super::parse_tls_response(&response_str)?;

        for (key, value) in parsed_response.cookies {
            self.cookies.set(key, value);
        }

        Ok(())
    }
}
