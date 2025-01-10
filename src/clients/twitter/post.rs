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
use rig::completion::Message;

impl<CM, EM> Client<CM, EM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
    EM: rig::embeddings::EmbeddingModel,
{
    pub async fn post(&mut self, topic: String) {
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
        let prompt = self.generate_post_prompt(topic, tweets);
        debug!("[TWITTER][POST] Generated prompt:\n{}", prompt);

        // Build the request for the completion model
        let request = self
            .agent
            .completion_model
            .completion_request(&prompt)
            .preamble(format!(
                "Your name: {}. Your Bio: {}. Use <characterInfo> and <timeline> to generate a Twitter post as @{}. Don't make your responses start like your previous ones.You MUST follow ALL the <rules>.",
                self.character.alias, self.character.bio, self.client.user.name
            ))
            .messages(self.post_history.iter().rev().cloned().collect())
            .build();

        match self.agent.completion(request).await {
            Ok(response) => {
                let agent_content = self.agent.response_extract_content(response);

                if !agent_content.is_empty() {
                    if self.config.debug {
                        info!("[TWITTER][DEBUG] Would have posted: {}", agent_content);
                    } else {
                        if let Err(e) = self.client.post(&agent_content).await {
                            error!("[TWITTER] Failed to post tweet: {}", e);
                        } else {
                            info!("[TWITTER] Successfully posted tweet");
                        }
                    }
                    self.push_post_history("user".to_string(), prompt);
                    self.push_post_history("assistant".to_string(), agent_content);
                }
            }
            Err(err) => error!(
                "[TWITTER][AGENT] Failed to generate completion: {}",
                err.to_string()
            ),
        }
    }

    fn generate_post_prompt(&self, topic: String, tweets: Vec<String>) -> String {
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
            
            Follow each step of <methodology> in chronological order processing each step and leveraging it into the next:
            <methodology>
            1) You are given <timeline> (A list of Twitter posts) as reference information to your <topic> to create a relevant message. Use this info to complete the post.
            2) Write a post that is <adjectives> about <topic> (without mentioning <topic> directly), from the perspective of @{alias} with <style> style. Try to write something totally different than previous posts. Do not add commentary or acknowledge this request, just write the post.
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

    fn push_post_history(&mut self, role: String, content: String) {
        if self.post_history.len() >= Self::POST_HISTORY_SIZE {
            self.post_history.pop_back();
        }
        self.post_history.push_front(Message {
            role: role,
            content: content,
        });
    }
}
