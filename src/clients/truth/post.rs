use super::{search::TruthDoc, Client};
use crate::{
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
        let prompt = self.generate_post_prompt(topic, truths);
        debug!("[TRUTH][POST] Generated prompt:\n{}", prompt);

        // Build the request for the completion model
        let request = self
            .agent
            .completion_model
            .completion_request(&prompt)
            .preamble(format!(
                "Your name: {}. Your Bio: {}. Use <characterInfo> and <timeline> to generate a Truth Social post as @{}. Don't make your responses start like your previous ones.You MUST follow ALL the <rules>.",
                self.character.alias, self.character.bio, self.client.user.username
            ))
            .messages(self.post_history.iter().rev().cloned().collect())
            .build();

        match self.agent.completion(request).await {
            Ok(response) => {
                let agent_content = self.agent.response_extract_content(response);

                if !agent_content.is_empty() {
                    if self.config.debug {
                        info!("[TRUTH][DEBUG] Would have posted: {}", agent_content);
                    } else {
                        if let Err(e) = self.client.post(agent_content.clone()).await {
                            error!("[TRUTH] Failed to post truth: {}", e);
                        } else {
                            info!("[TRUTH] Successfully posted truth");
                        }
                    }
                    self.push_post_history("user".to_string(), prompt);
                    self.push_post_history("assistant".to_string(), agent_content);
                }
            }
            Err(err) => error!(
                "[TRUTH][AGENT] Failed to generate completion: {}",
                err.to_string()
            ),
        }
    }

    fn generate_post_prompt(&self, topic: String, truths: Vec<String>) -> String {
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
            1) You are given <timeline> (A list of Truth Social posts) as reference information to your <topic> to create a relevant message. Use this info to complete the post.
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
            alias = self.client.user.username,
            adjectives = self.character.choose_random_traits(Adjectives, 3),
            lore = self.character.choose_random_traits(Lore, 3),
            style = self.character.choose_random_traits(Styles, 1),
            inspirations = self.character.choose_random_traits(Inspirations, 3),
            timeline = truths.join("\n")
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
