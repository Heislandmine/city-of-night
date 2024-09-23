use crate::core::{
    actions::TextMessage, character::Character, game_data::CharactersAvailableForPurchase,
};

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

pub struct PurchaseCharacterRenderContext {
    pub character_list_available_for_purchase: Vec<CharactersAvailableForPurchase>,
    pub message: Option<TextMessage>,
}

impl PurchaseCharacterRenderContext {
    pub fn new(
        character_list_available_for_purchase: Vec<CharactersAvailableForPurchase>,
        message: Option<TextMessage>,
    ) -> Self {
        Self {
            character_list_available_for_purchase,
            message,
        }
    }
}
