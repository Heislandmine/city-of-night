use crate::core::{actions::TextMessage, character::Character};

pub struct HomeRenderContext {
    pub breaking_character: Option<Character>,
    pub message: Option<TextMessage>,
}

impl HomeRenderContext {
    pub fn new(breaking_character: Option<Character>, message: Option<TextMessage>) -> Self {
        Self {
            breaking_character,
            message,
        }
    }
}
