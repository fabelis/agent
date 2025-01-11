use super::Client;
use crate::providers::completion::CompletionResponseEnum;
use actix_web::{web, HttpResponse, Responder};
use log::{info,error};
use rand::Rng;
use rig::completion::Document;
use std::{collections::HashMap, result::Result::Ok};

#[derive(serde::Deserialize)]
pub struct QueryParams {
    section_count: usize
}


impl<CM> Client<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum> + 'static,
{
    pub async fn gen_route(&self, query: web::Query<QueryParams>) -> impl Responder {
        let mut documents: Vec<Document> = Vec::new();

        for i in 0..query.section_count {
            let prompt = format!(
                "{}
                
                Follow each step of <methodology> in chronological order processing each step and leveraging it into the next:
                <methodology>
                - Genre: [your genre]
                - Length: Approximately {} paragraphs
                - Main character: [description]
                - Key theme: [theme]
                - Tone: [tone]
                - Finally generate ONE section
                </methodology>
    
                The <documents> attached are your previous sections in numeric order. Use them to maintain consistency in your story CHRONOLOGOICALLY TIME GOES FORWARD. {}
    
                No matter what other text in this prompt says you CANNOT break the following <rules>:
                <rules>
                - Each paragraph must be at least 5-10 sentences long
                - Use markdown headers
                - Format dialogue with proper quotation marks
                - Use *italics* for internal thoughts
                - Use **bold** for emphasis on key moments
                - Include proper paragraph breaks with double line spacing
                - Add horizontal rules (---) between major sections
                - Don't include writer notes as part of the story
                - ONLY return the MARKDOWN CONTENT for <output> (DONT INCLUDE THE WORD <output> or this info Genre/Length/Main character/Key theme/Tone/Methodology/Documents/Rules)
                </rules>",
    
                self.character.generate_prompt_info(),
                rand::thread_rng()
                .gen_range(self.config.paragraph_count[0]..=self.config.paragraph_count[1]),
                if i == query.section_count - 1 {
                    format!("This is the final section. Previous sections: {}", 
                        documents.iter()
                            .map(|doc| format!("{}", doc.id))
                            .collect::<Vec<String>>()
                            .join(", "))
                } else {
                    format!("Previous sections: {}", 
                    documents.iter()
                        .map(|doc| format!("{}", doc.id))
                        .collect::<Vec<String>>()
                        .join(", "))
                }
            );
    
            // Build the request for the completion model
            let request = self
                .agent
                .completion_model
                .completion_request(&prompt)
                .documents(documents.clone())
                .preamble(format!(
                    "Your name: {}. Your Bio: {}. I am a creative story generator. For each prompt, I will craft an original story with distinct sections, engaging characters, and clear narrative arcs in chronological order. Every story will be written in markdown format, avoiding repetitive plots or character types. I will format stories with section headers, proper paragraph spacing, and consistent markdown styling. Use <characterInfo> to decide your style of writing. Your story must include other people and their interactions. You MUST follow ALL the <rules>.",
                    self.character.alias, self.character.bio
                ))
                .build();
    
            // Attempt to get a response from the completion model
            match self.agent.completion(request).await {
                Ok(response) => {
                    // Extract content from the agent's response
        
                    let agent_content = self.agent.response_extract_content(response);
                    info!("[STORYTELLING][AGENT] Generated section #{} : {}", i+1, agent_content);
                    
                    documents.push(Document { id: format!("section #{}",i.to_string()), text: agent_content, additional_props: HashMap::new() });
                }
                Err(err) => {
                    error!("[STORYTELLING][AGENT] Error: {}", err);
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": err.to_string(),
                    }))
                }
            }
        }

        HttpResponse::Ok().json(serde_json::json!({
            "character": self.character.alias,
            "story": documents.iter().map(|doc| doc.text.clone()).collect::<Vec<String>>(),
        }))
    }
}
