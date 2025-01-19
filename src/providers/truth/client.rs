use super::{cookies::Cookies, Profile};
use native_tls::TlsConnector;
use std::net::TcpStream;

#[derive(Clone)]
pub struct Client {
    pub connector: TlsConnector,
    pub access_token: String,
    pub cookies: Cookies,
    pub user: Profile,
}

impl Client {
    pub fn new() -> Self {
        Client {
            connector: TlsConnector::new().unwrap(),
            access_token: String::new(),
            cookies: Cookies::new(),
            user: Profile {
                id: String::new(),
                username: String::new(),
                acct: String::new(),
                display_name: String::new(),
                locked: false,
                bot: false,
                discoverable: false,
                group: false,
                created_at: String::new(),
                note: String::new(),
                url: String::new(),
                avatar: String::new(),
                avatar_static: String::new(),
                header: String::new(),
                header_static: String::new(),
                followers_count: 0,
                following_count: 0,
                statuses_count: 0,
                last_status_at: String::new(),
                verified: false,
                location: String::new(),
                website: String::new(),
                accepting_messages: false,
                show_nonmember_group_statuses: None,
                emojis: Vec::new(),
                fields: Vec::new(),
            },
        }
    }

    pub fn create_tls_stream(self) -> Result<native_tls::TlsStream<TcpStream>, anyhow::Error> {
        let tcp_stream = TcpStream::connect("truthsocial.com:443")?;
        let tls_stream = self.connector.connect("truthsocial.com", tcp_stream)?;
        Ok(tls_stream)
    }
}
