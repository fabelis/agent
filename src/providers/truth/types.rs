use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Profile {
    pub id: String,
    pub username: String,
    pub acct: String,
    pub display_name: String,
    pub locked: bool,
    pub bot: bool,
    pub discoverable: bool,
    pub group: bool,
    pub created_at: String,
    pub note: String,
    pub url: String,
    pub avatar: String,
    pub avatar_static: String,
    pub header: String,
    pub header_static: String,
    pub followers_count: i64,
    pub following_count: i64,
    pub statuses_count: i64,
    pub last_status_at: Option<String>,
    pub verified: bool,
    pub location: String,
    pub website: String,
    pub accepting_messages: bool,
    pub show_nonmember_group_statuses: Option<bool>,
    pub emojis: Vec<String>,
    pub fields: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TrendHistory {
    pub day: String,
    pub uses: String,
    pub accounts: String,
    pub days_ago: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Trend {
    pub url: String,
    pub name: String,
    pub history: Vec<TrendHistory>,
    pub recent_history: Vec<i32>,
    pub recent_statuses_count: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Post {
    pub id: String,
    pub created_at: String,
    pub edited_at: Option<String>,
    pub in_reply_to_id: Option<String>,
    pub quote_id: Option<String>,
    pub in_reply_to_account_id: Option<String>,
    pub visibility: String,
    pub language: String,
    pub uri: String,
    pub url: String,
    pub content: String,
    pub text: Option<String>,
    pub account: Profile,
    pub media_attachments: Option<Vec<MediaAttachment>>,
    pub mentions: Vec<PostMention>,
    pub tags: Vec<String>,
    pub card: Option<serde_json::Value>,
    pub group: Option<serde_json::Value>,
    pub quote: Option<serde_json::Value>,
    pub in_reply_to: Option<Box<Post>>,
    pub reblog: Option<serde_json::Value>,
    pub replies_count: i64,
    pub reblogs_count: i64,
    pub favourites_count: i64,
    pub favourited: bool,
    pub reblogged: bool,
    pub muted: bool,
    pub pinned: Option<bool>,
    pub bookmarked: bool,
    pub poll: Option<serde_json::Value>,
    pub emojis: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MediaAttachment {
    pub id: String,
    pub url: String,
    pub preview_url: String,
    pub remote_url: Option<String>,
    pub meta: Option<serde_json::Value>,
    pub description: Option<String>,
    pub blurhash: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PostMention {
    pub id: String,
    pub username: String,
    pub url: String,
    pub acct: String,
}
