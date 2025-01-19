use log::info;

use super::{client::Client, Post};
use std::{
    collections::HashMap,
    io::{Read, Write},
};

impl Client {
    pub async fn get_posts_replies(
        &mut self,
        posts_ids: Vec<String>,
    ) -> Result<HashMap<String, Vec<Post>>, anyhow::Error> {
        let mut replies_map = HashMap::new();

        for post_id in posts_ids {
            let replies = self.get_post_replies(post_id.clone()).await?;
            replies_map.insert(post_id, replies.clone()); // Get first reply for each post
        }

        Ok(replies_map)
    }

    pub async fn get_post_replies(&mut self, post_id: String) -> Result<Vec<Post>, anyhow::Error> {
        let mut stream = self.clone().create_tls_stream()?;

        let request = format!(
            "GET /api/v2/statuses/{}/context/descendants?sort=trending HTTP/1.1\r\n\
            Host: truthsocial.com\r\n\
            Accept: application/json, text/plain, */*\r\n\
            Accept-Language: en-US,en;q=0.9\r\n\
            Authorization: Bearer {}\r\n\
            Cookie: {}\r\n\
            Priority: u=1, i\r\n\
            Referer: https://truthsocial.com/\r\n\
            Sec-CH-UA: \"Google Chrome\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\"\r\n\
            Sec-CH-UA-Mobile: ?0\r\n\
            Sec-CH-UA-Platform: \"Windows\"\r\n\
            Sec-Fetch-Dest: empty\r\n\
            Sec-Fetch-Mode: cors\r\n\
            Sec-Fetch-Site: same-origin\r\n\
            User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36\r\n\
            Connection: close\r\n\
            \r\n",
            post_id,
            self.access_token,
            self.cookies.format()
        );

        stream.write_all(request.as_bytes())?;

        let mut response = Vec::new();
        stream.read_to_end(&mut response)?;

        let response_str = String::from_utf8(response)?;

        let parsed_response = super::parse_tls_response(&response_str)?;

        for (key, value) in parsed_response.cookies {
            self.cookies.set(key, value);
        }

        let replies: Vec<Post> = serde_json::from_str(&parsed_response.body)?;

        Ok(replies)
    }
}
