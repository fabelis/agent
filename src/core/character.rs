use rand::seq::SliceRandom;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct Character {
    pub alias: String,
    pub bio: String,
    pub adjectives: Vec<String>,
    pub lore: Vec<String>,
    pub styles: Vec<String>,
    pub topics: Vec<String>,
    pub inspirations: Vec<String>,
    #[serde(skip)]
    pub path: String,
}

impl Character {
    pub fn new(path: String) -> Self {
        Character {
            alias: "".to_string(),
            bio: "".to_string(),
            adjectives: vec![],
            lore: vec![],
            styles: vec![],
            topics: vec![],
            inspirations: vec![],
            path,
        }
    }

    pub fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(self.path.clone())?;
        *self = serde_json::from_str(&content)?;
        Ok(())
    }

    pub fn choose_random_traits(&self, trait_type: CharacterTrait, count: usize) -> String {
        match trait_type {
            CharacterTrait::Adjectives => self
                .adjectives
                .choose_multiple(&mut rand::thread_rng(), count),
            CharacterTrait::Lore => self.lore.choose_multiple(&mut rand::thread_rng(), count),
            CharacterTrait::Styles => self.styles.choose_multiple(&mut rand::thread_rng(), count),
            CharacterTrait::Topics => self.topics.choose_multiple(&mut rand::thread_rng(), count),
            CharacterTrait::Inspirations => self
                .inspirations
                .choose_multiple(&mut rand::thread_rng(), count),
        }
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .join(", ")
    }

    pub fn generate_prompt_info(&self) -> String {
        format!(
            "<characterInfo>
            These describe you:
            <adjectives>
            {}
            </adjectives>
            This has happened to you:
            <lore>
            {}
            </lore>
            You are known for these styles:
            <styles>
            {}
            </styles>
            You are interested in these topics:
            <topics>
            {}
            </topics>
            You are inspired by these:
            <inspirations>
            {}
            </inspirations>
            </characterInfo>",
            self.choose_random_traits(CharacterTrait::Adjectives, 3),
            self.choose_random_traits(CharacterTrait::Lore, 3),
            self.choose_random_traits(CharacterTrait::Styles, 3),
            self.choose_random_traits(CharacterTrait::Topics, 3),
            self.choose_random_traits(CharacterTrait::Inspirations, 3)
        )
    }
}

#[derive(Clone, Copy)]
pub enum CharacterTrait {
    Adjectives,
    Lore,
    Styles,
    Topics,
    Inspirations,
}
