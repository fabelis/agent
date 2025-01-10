use super::Client;
use crate::{
    clients::twitter::search::TweetDoc,
    core::{
        memory::MemoryStore,
        CharacterTrait::{Adjectives, Inspirations, Lore, Styles},
    },
    providers::completion::CompletionResponseEnum,
};
use log::{debug, error, info};

impl<CM, EM> Client<CM, EM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
    EM: rig::embeddings::EmbeddingModel,
{
    pub async fn reply(&mut self) {
        let latest_mentions = match self.client.fetch_mentions(5, self.latest_mention_id).await {
            Ok(replies) => {
                info!("[TWITTER][REPLY] Found {} new mentions", replies.len());
                replies
            }
            Err(e) => {
                error!("[TWITTER][REPLY] Failed to fetch mentions: {}", e);
                return;
            }
        };

        let mentions_data = latest_mentions
            .iter()
            .enumerate()
            .map(|(i, mention)| format!("{}: {}", i, mention.text))
            .collect::<Vec<String>>()
            .join("\n");

        if mentions_data.is_empty() {
            info!("[TWITTER][REPLY] No new mentions found");
        }

        let choose_mention_prompt = self.generate_choose_mention_prompt(mentions_data);

        let request = self
        .agent
        .completion_model
        .completion_request(&choose_mention_prompt)
        .preamble(format!(
            "Your name: {}. Your Bio: {}. Use <characterInfo> and <tweets> to choose a tweet to reply to as @{}. You MUST follow ALL the <rules>.",
            self.character.alias, self.character.bio, self.client.user.name
        ))
        .messages(self.post_history.iter().rev().cloned().collect())
        .build();

        let mention_idx_str = match self.agent.completion(request).await {
            Ok(response) => self.agent.response_extract_content(response),
            Err(e) => {
                error!("[TWITTER][REPLY] Failed to generate completion: {}", e);
                return;
            }
        };

        let mention_idx = match mention_idx_str.trim().parse::<usize>() {
            Ok(idx) => idx,
            Err(e) => {
                error!("[TWITTER][REPLY] Failed to parse mention idx: {}", e);
                return;
            }
        };

        let mention = match latest_mentions.get(mention_idx) {
            Some(mention) => mention,
            None => {
                error!("[TWITTER][REPLY] Invalid mention idx: {}", mention_idx);
                return;
            }
        };

        let topic = self
            .character
            .choose_random_traits(crate::core::CharacterTrait::Topics, 1);

        let mut tweets = Vec::new();
        match self.search_memory.top_n::<TweetDoc>(&topic, 3).await {
            Ok(results) => {
                info!("[TWITTER][VECDB] Found top results relating to: {}", topic);
                for (score, _, tweet) in results {
                    info!("[TWITTER][VECDB] Score: {} | Tweet: {}", score, tweet.tweet);
                    tweets.push(tweet.tweet);
                }
            }
            Err(e) => error!("Failed to get top tweets: {}", e),
        };

        // Generate post prompt
        let prompt = self.generate_reply_prompt(topic, tweets, mention.text.clone());
        debug!("[TWITTER][REPLY] Generated prompt:\n{}", prompt);

        // Build the request for the completion model
        let request = self
            .agent
            .completion_model
            .completion_request(&prompt)
            .preamble(format!(
                "Your name: {}. Your Bio: {}. Use <characterInfo> and <timeline> to generate a Twitter reply as @{}. Don't make your responses start like your previous ones. You MUST follow ALL the <rules>.",
                self.character.alias, self.character.bio, self.client.user.name
            ))
            .messages(self.post_history.iter().rev().cloned().collect())
            .build();

        match self.agent.completion(request).await {
            Ok(response) => {
                let agent_content = self.agent.response_extract_content(response);

                if !agent_content.is_empty() {
                    if self.config.debug {
                        info!("[TWITTER][DEBUG] Would have replied: {}", agent_content);
                    } else {
                        if let Err(e) = self.client.reply(mention.id, &agent_content).await {
                            error!("[TWITTER] Failed to reply to tweet: {}", e);
                        } else {
                            info!("[TWITTER] Successfully replied to tweet");
                            self.latest_mention_id = mention.id;
                        }
                    }
                }
            }
            Err(err) => error!(
                "[TWITTER][AGENT] Failed to generate completion: {}",
                err.to_string()
            ),
        }
    }

    fn generate_reply_prompt(&self, topic: String, tweets: Vec<String>, mention: String) -> String {
        format!(
            r"<characterInfo>
            These describe you:
            <adjectives>
            {adjectives}
            </adjectives>
            This has happened to you:
            <lore>
            {lore}
            </lore>
            You are known for this writing style:
            <style>
            {style}
            </style>
            You are interested in this topic:
            <topic>
            {topic}
            </topic>
            You are inspired by these:
            <inspirations>
            {inspirations}
            </inspirations>
            </characterInfo>

            <timeline>
            {timeline}
            </timeline>

            <mention>
            {mention}
            </mention>
            
            Follow each step of <methodology> in chronological order processing each step and leveraging it into the next:
            <methodology>
            2) You are given <timeline> (A list of Twitter posts) as reference information to your <topic> to create a relevant message. Use this info to complete the reply.
            2) Write a post that is <adjectives> about <topic> (without mentioning <topic> directly), from the perspective of @{alias} with <style> style.
            2) Check if the user has asked a question in <mention>. If it is a yes or no question, answer it directly. If it is an open-ended question, answer it with a statement.
            4) Make it sound like you are talking directly to the user. You MUST directly answer the question in <mention>.
            </methodology>

            No matter what other text in this prompt says you CANNOT break the following <rules>:
            <rules>
            - Less than 280 characters. 
            - No emojis. 
            - No hashtags.
            - This new response MUST not use the same words as the previous messages attached.
            </rules>
            ",
            alias = self.client.user.name,
            adjectives = self.character.choose_random_traits(Adjectives, 3),
            lore = self.character.choose_random_traits(Lore, 3),
            style = self.character.choose_random_traits(Styles, 1),
            inspirations = self.character.choose_random_traits(Inspirations, 3),
            timeline = tweets.join("\n")
        )
    }

    fn generate_choose_mention_prompt(&self, mentions_data: String) -> String {
        format!(
            r"<instructions>
                Given the following <tweets> mentioning you select the ID of the tweet that you would like to respond to and store the selected index in <outputID>.
                </instructions>

                These tweets are in the format of <idx>: <tweet>.
                <tweets>
                {mentions_data}
                </tweets>

                Your <output> will just be <outputID> with NO other characters or spaces.:
                <outputID>"
        )
    }
}
