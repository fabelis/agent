use super::{client::Client, Profile};
use log::info;
use serde::Deserialize;
use std::io::{Read, Write};

#[derive(Deserialize, Debug)]
struct LoginResponse {
    access_token: String,
    token_type: String,
    scope: String,
    created_at: u64,
}

#[derive(Deserialize, Debug)]
struct VerifyResponseSource {
    privacy: String,
    sensitive: bool,
    language: Option<String>,
    email: String,
    approved: bool,
    note: String,
    fields: Vec<String>, // Assuming an array of strings
    sms_verified: bool,
    ready_by_sms_verification: bool,
    follow_requests_count: i64,
    accepting_messages: bool,
    chats_onboarded: bool,
    feeds_onboarded: bool,
    tv_onboarded: bool,
    bookmarks_onboarded: bool,
    show_nonmember_group_statuses: bool,
    unauth_visibility: bool,
    integrity: i64,
    integrity_status: Vec<String>,
    sms_reverification_required: bool,
    sms: bool,
    sms_country: String,
    receive_only_follow_mentions: bool,
    email_verified: bool,
    accepted_status_edit_prompt: bool,
    unapproved_position: i64,
    sms_last_four_digits: String,
}

#[derive(Deserialize, Debug)]
struct VerifyResponseFeatures {
    for_you: bool,
    tv: bool,
}

impl Client {
    pub async fn login(&mut self, username: String, password: String) -> Result<(), anyhow::Error> {
        self.login_req(username, password).await?;
        self.verify_req().await?;

        Ok(())
    }

    async fn login_req(&mut self, username: String, password: String) -> Result<(), anyhow::Error> {
        let mut stream = self.clone().create_tls_stream()?;

        let body = format!(
            r#"
        {{
            "client_id": "9X1Fdd-pxNsAgEDNi_SfhJWi8T-vLuV2WVzKIbkTCw4",
            "client_secret": "ozF8jzI4968oTKFkEnsBC-UbLPCdrSv0MkXGQu2o_-M",
            "redirect_uri": "urn:ietf:wg:oauth:2.0:oob",
            "grant_type": "password",
            "scope": "read write follow push",
            "username": "{}",
            "password": "{}"
        }}
        "#,
            username, password
        );

        let request = format!(
            "POST /oauth/token HTTP/1.1\r\n\
            Host: truthsocial.com\r\n\
            Accept: application/json, text/plain, */*\r\n\
            Accept-Language: en-US,en;q=0.9\r\n\
            Content-Type: application/json\r\n\
            Origin: https://truthsocial.com\r\n\
            Priority: u=1, i\r\n\
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

        let login_response: LoginResponse = serde_json::from_str(&parsed_response.body)?;
        if login_response.access_token.len() == 0 {
            return Err(anyhow::anyhow!("Failed to locate access token"));
        }

        self.access_token = login_response.access_token;

        Ok(())
    }

    async fn verify_req(&mut self) -> Result<(), anyhow::Error> {
        let mut stream = self.clone().create_tls_stream()?;

        let request = format!(
            "GET /api/v1/accounts/verify_credentials HTTP/1.1\r\n\
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

        let verify_response: Profile = serde_json::from_str(&parsed_response.body)?;

        if verify_response.id.len() == 0 {
            return Err(anyhow::anyhow!("Failed to locate user ID"));
        }

        self.user = verify_response;

        Ok(())
    }
}
