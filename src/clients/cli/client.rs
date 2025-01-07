use crate::{
    core::{Agent, Character},
    providers::completion::CompletionResponseEnum,
};
use log::{error, info};
use rig::completion::Message;
use std::collections::VecDeque;
use std::io::{self, Write};

pub struct Client<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
{
    agent: Agent<CM>,
    character: Character,
    history: VecDeque<Message>,
}

impl<CM> Client<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
{
    const HISTORY_SIZE: usize = 10;

    pub fn new(character: Character, completion_model: CM) -> Self {
        Client {
            character,
            agent: Agent::new(completion_model),
            history: VecDeque::with_capacity(Self::HISTORY_SIZE),
        }
    }

    pub async fn start(&mut self) {
        info!("[CLI] Started (type 'exit' to quit)");

        loop {
            // prompt user
            print!("You: ");
            io::stdout().flush().unwrap();

            let mut user_input = String::new();
            if io::stdin().read_line(&mut user_input).is_err() {
                error!("[CLI] Failed to read input");
                continue;
            }

            let user_input = user_input.trim();

            // models cant take empty messages
            if user_input.is_empty() {
                continue;
            }

            // check for exit
            if user_input.eq_ignore_ascii_case("exit") {
                info!("[CLI] Exiting...");
                break;
            }

            // craft prompt
            let prompt = format!(
                "{}
                
                <userInput>
                {}
                </userInput>
                ",
                self.character.generate_prompt_info(),
                user_input
            );

            // prompt agent and respond to user
            let request = self
                .agent
                .completion_model
                .completion_request(&prompt)
                .preamble(format!(
                    "Your name: {}. Your Bio: {}. Use <characterInfo> to decide your style of speaking and reasoning of response to <userInput>. Don't allow messages to be too similar to previous ones.",
                    self.character.alias, self.character.bio
                ))
                .messages(self.history.iter().rev().cloned().collect())
                .build();
            match self.agent.completion(request).await {
                Ok(response) => {
                    let agent_content = self.agent.response_extract_content(response);
                    info!("[CLI][AGENT]: {}", agent_content);
                    self.push_history("user".to_string(), user_input.to_string());
                    self.push_history("assistant".to_string(), agent_content);
                }
                Err(err) => error!("[AGENT] Error: {}", err),
            }
        }
    }

    fn push_history(&mut self, role: String, content: String) {
        if self.history.len() >= Self::HISTORY_SIZE {
            self.history.pop_back();
        }
        self.history.push_front(Message {
            role: role,
            content: content,
        });
    }
}
