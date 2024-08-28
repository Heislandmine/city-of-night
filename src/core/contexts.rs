use super::game_data::CharactersAvailableForPurchase;

pub struct RenderContext {
    pub character_list_available_for_purchase: Vec<CharactersAvailableForPurchase>,
    pub string_user_inputted: String,
}

impl RenderContext {
    pub fn new(
        character_list_available_for_purchase: Vec<CharactersAvailableForPurchase>,
        string_user_inputted: String,
    ) -> Self {
        Self {
            character_list_available_for_purchase,
            string_user_inputted,
        }
    }
}
