use anyhow::{Error, Result};
use std::env;
use twitter_v2::{authorization::Oauth1aToken, id::NumericId, query, Tweet, TwitterApi, User};

#[derive(Clone)]
pub struct Client {
    api: TwitterApi<Oauth1aToken>,
    pub user: User,
}

impl Client {
    pub async fn new() -> Self {
        // load vars
        let api_key = env::var("TWITTER_API_KEY").expect("TWITTER_API_KEY not set");
        let api_secret = env::var("TWITTER_API_SECRET").expect("TWITTER_API_SECRET not set");
        let access_token = env::var("TWITTER_ACCESS_TOKEN").expect("TWITTER_ACCESS_TOKEN not set");
        let access_token_secret =
            env::var("TWITTER_ACCESS_TOKEN_SECRET").expect("TWITTER_ACCESS_TOKEN_SECRET not set");

        // create api handler
        let api = TwitterApi::new(Oauth1aToken::new(
            api_key,
            api_secret,
            access_token,
            access_token_secret,
        ));

        let user = api
            .get_users_me()
            .send()
            .await
            .unwrap()
            .into_data()
            .expect("[TWITTER] Failed to get userID");

        Self { api, user }
    }

    pub async fn post(&self, response: &str) -> Result<Tweet> {
        Ok(self
            .api
            .post_tweet()
            .text(response.to_string())
            .send()
            .await?
            .into_data()
            .ok_or_else(|| Error::msg("Failed to post tweet"))?)
    }

    pub async fn reply(&self, id: NumericId, response: &str) -> Result<Tweet> {
        Ok(self
            .api
            .post_tweet()
            .in_reply_to_tweet_id(id)
            .text(response.to_string())
            .send()
            .await?
            .into_data()
            .ok_or_else(|| Error::msg("failed to get reply data"))?)
    }

    pub async fn fetch_mentions(&self, count: usize, latest_id: NumericId) -> Result<Vec<Tweet>> {
        Ok(self
            .api
            .get_user_mentions(self.user.id)
            .since_id(latest_id)
            .max_results(count)
            .send()
            .await?
            .into_data()
            .ok_or_else(|| Error::msg("Failed to fetch mentions"))?)
    }

    pub async fn fetch_timeline(&self, count: usize, latest_id: NumericId) -> Result<Vec<Tweet>> {
        Ok(self
            .api
            .get_user_tweets(self.user.id)
            .since_id(latest_id)
            .max_results(count)
            .send()
            .await?
            .into_data()
            .ok_or_else(|| Error::msg("Failed to fetch timeline"))?)
    }

    pub async fn search(&self, query: String) -> Result<Vec<Tweet>> {
        Ok(self
            .api
            .get_tweets_search_recent(query)
            .sort_order(query::SortOrder::Relevancy)
            .max_results(10)
            .send()
            .await?
            .into_data()
            .ok_or_else(|| Error::msg("Failed to fetch search results"))?)
    }
}
