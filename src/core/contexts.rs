use super::{
    actions::TextMessage, character::Character, game_data::CharactersAvailableForPurchase,
};

pub struct RenderContext {
    pub character_list_available_for_purchase: Vec<CharactersAvailableForPurchase>,
    pub message: Option<TextMessage>,
    pub breaking_character: Option<Character>,
}

impl RenderContext {
    pub fn new(
        character_list_available_for_purchase: Vec<CharactersAvailableForPurchase>,
        message: Option<TextMessage>,
        breaking_character: Option<Character>,
    ) -> Self {
        Self {
            character_list_available_for_purchase,
            message,
            breaking_character,
        }
    }
}
