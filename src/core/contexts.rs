use super::{actions::TextMessage, game_data::CharactersAvailableForPurchase};

pub struct RenderContext {
    pub character_list_available_for_purchase: Vec<CharactersAvailableForPurchase>,
    pub message: Option<TextMessage>,
}

impl RenderContext {
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
