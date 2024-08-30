use super::game_data::CharactersAvailableForPurchase;

pub struct RenderContext {
    pub character_list_available_for_purchase: Vec<CharactersAvailableForPurchase>,
    pub message: Option<String>,
}

impl RenderContext {
    pub fn new(
        character_list_available_for_purchase: Vec<CharactersAvailableForPurchase>,
        message: Option<String>,
    ) -> Self {
        Self {
            character_list_available_for_purchase,
            message,
        }
    }
}
