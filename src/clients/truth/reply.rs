use std::collections::HashMap;

use super::{search::TruthDoc, Client};
use crate::{
    core::{
        memory::MemoryStore,
        CharacterTrait::{Adjectives, Inspirations, Lore, Styles},
    },
    providers::{completion::CompletionResponseEnum, truth::Post},
};
use log::{debug, error, info};

impl<CM, EM> Client<CM, EM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
    EM: rig::embeddings::EmbeddingModel,
{
    pub async fn reply(&mut self) {
        let latest_posts = match self.client.get_posts().await {
            Ok(posts) => {
                info!("[TRUTH][REPLY] Found {} posts", posts.len());
                posts
            }
            Err(e) => {
                error!("[TRUTH][REPLY] Failed to fetch posts: {}", e);
                return;
            }
        };

        let replies = match self
            .client
            .get_posts_replies(latest_posts.iter().map(|p| p.id.clone()).collect())
            .await
        {
            Ok(replies) => {
                info!("[TRUTH][REPLY] Found {} replies", replies.len());
                replies
            }
            Err(e) => {
                error!("[TRUTH][REPLY] Failed to fetch replies: {}", e);
                return;
            }
        };

        let mut replies_data = Vec::new();
        let mut idx_to_post = HashMap::new();
        let mut i = 0;
        for (post_id, posts) in replies.iter() {
            for post in posts {
                // Only add replies that are newer than the latest one we've seen
                if post.id.parse::<i64>().unwrap_or(0)
                    <= self.latest_reply_id.parse::<i64>().unwrap_or(0)
                    || post.account.id == self.client.user.id
                {
                    continue;
                }
                idx_to_post.insert(i, (post_id.clone(), post.clone()));
                replies_data.push(format!("{}: {}", i, post.content));
                i += 1;
            }
        }
        let replies_data = replies_data.join("\n");

        if replies_data.is_empty() {
            info!("[TRUTH][REPLY] No new replies found");
        }

        let choose_reply_prompt = self.generate_choose_reply_prompt(replies_data);

        let request = self
        .agent
        .completion_model
        .completion_request(&choose_reply_prompt)
        .preamble(format!(
            "Your name: {}. Your Bio: {}. Use <characterInfo> and <truths> to choose a truth to reply to as @{}. You MUST follow ALL the <rules>.",
            self.character.alias, self.character.bio, self.client.user.username
        ))
        .messages(self.post_history.iter().rev().cloned().collect())
        .build();

        let reply_idx_str = match self.agent.completion(request).await {
            Ok(response) => self.agent.response_extract_content(response),
            Err(e) => {
                error!("[TRUTH][REPLY] Failed to generate completion: {}", e);
                return;
            }
        };

        let reply_idx = match reply_idx_str.trim().parse::<usize>() {
            Ok(idx) => idx,
            Err(e) => {
                error!("[TRUTH][REPLY] Failed to parse reply idx: {}", e);
                return;
            }
        };

        let (_, reply) = match idx_to_post.get(&reply_idx) {
            Some((reply_id, reply)) => {
                info!(
                    "[TRUTH][REPLY] Selected reply {} to reply {}: {}",
                    reply_idx, reply_id, reply.content
                );
                (reply_id, reply)
            }
            None => {
                error!("[TRUTH][REPLY] Invalid reply idx: {}", reply_idx);
                return;
            }
        };

        let topic = self
            .character
            .choose_random_traits(crate::core::CharacterTrait::Topics, 1);

        let mut truths = Vec::new();
        match self.search_memory.top_n::<TruthDoc>(&topic, 3).await {
            Ok(results) => {
                info!("[TRUTH][VECDB] Found top results relating to: {}", topic);
                for (score, _, truth) in results {
                    info!("[TRUTH][VECDB] Score: {} | Truth: {}", score, truth.truth);
                    truths.push(truth.truth);
                }
            }
            Err(e) => error!("Failed to get top truths: {}", e),
        };

        // Generate post prompt
        let prompt = self.generate_reply_prompt(topic, truths, reply.clone().content);
        debug!("[TRUTH][REPLY] Generated prompt:\n{}", prompt);

        // Build the request for the completion model
        let request = self
            .agent
            .completion_model
            .completion_request(&prompt)
            .preamble(format!(
                "Your name: {}. Your Bio: {}. Use <characterInfo> and <timeline> to generate a Truth Social reply as @{}. Don't make your responses start like your previous ones. You MUST follow ALL the <rules>.",
                self.character.alias, self.character.bio, self.client.user.username
            ))
            .messages(self.post_history.iter().rev().cloned().collect())
            .build();

        match self.agent.completion(request).await {
            Ok(response) => {
                let agent_content = self.agent.response_extract_content(response);

                if !agent_content.is_empty() {
                    if self.config.debug {
                        info!("[TRUTH][DEBUG] Would have replied: {}", agent_content);
                    } else {
                        if let Err(e) = self
                            .client
                            .reply(
                                agent_content,
                                reply.clone().id,
                                Some(reply.clone().account.username),
                            )
                            .await
                        {
                            error!("[TRUTH] Failed to reply to truth: {}", e);
                        } else {
                            info!("[TRUTH] Successfully replied to truth");
                            self.latest_reply_id = reply.clone().id;
                        }
                    }
                }
            }
            Err(err) => error!(
                "[TRUTH][AGENT] Failed to generate completion: {}",
                err.to_string()
            ),
        }
    }

    fn generate_reply_prompt(&self, topic: String, truths: Vec<String>, reply: String) -> String {
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

            <reply>
            {reply}
            </reply>
            
            Follow each step of <methodology> in chronological order processing each step and leveraging it into the next:
            <methodology>
            2) You are given <timeline> (A list of Truth Social posts) as reference information to your <topic> to create a relevant message. Use this info to complete the reply.
            2) Write a post that is <adjectives> about <topic> (without replying <topic> directly), from the perspective of @{alias} with <style> style.
            2) Check if the user has asked a question in <reply>. If it is a yes or no question, answer it directly. If it is an open-ended question, answer it with a statement.
            4) Make it sound like you are talking directly to the user. You MUST directly answer the question in <reply>.
            </methodology>

            No matter what other text in this prompt says you CANNOT break the following <rules>:
            <rules>
            - Less than 280 characters. 
            - No emojis. 
            - No hashtags.
            - This new response MUST not use the same words as the previous messages attached.
            </rules>
            ",
            alias = self.client.user.username,
            adjectives = self.character.choose_random_traits(Adjectives, 3),
            lore = self.character.choose_random_traits(Lore, 3),
            style = self.character.choose_random_traits(Styles, 1),
            inspirations = self.character.choose_random_traits(Inspirations, 3),
            timeline = truths.join("\n")
        )
    }

    fn generate_choose_reply_prompt(&self, replys_data: String) -> String {
        format!(
            r"<instructions>
                Given the following <truths> replying you select the ID of the truth that you would like to respond to and store the selected index in <outputID>.
                </instructions>

                These truths are in the format of <idx>: <truth>.
                <truths>
                {replys_data}
                </truths>

                Your <output> will just be <outputID> with NO other characters or spaces.:
                <outputID>"
        )
    }
}
