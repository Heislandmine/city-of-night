use super::game_data::{CharactersAvailableForPurchase, GameWorld, UserInventory};

pub struct GameController {
    user_inventory: UserInventory,
    world: GameWorld,
    character_list_available_for_purchase: Vec<CharactersAvailableForPurchase>,
}

impl GameController {
    pub fn new(
        user_inventory: UserInventory,
        world: GameWorld,
        character_list_available_for_purchase: Vec<CharactersAvailableForPurchase>,
    ) -> Self {
        Self {
            user_inventory,
            world,
            character_list_available_for_purchase,
        }
    }

    pub fn get_character_list_available_for_purchase(&self) -> Vec<CharactersAvailableForPurchase> {
        let filtered_list = self
            .character_list_available_for_purchase
            .iter()
            .filter(|character| {
                !self
                    .user_inventory
                    .owned_characters()
                    .contains(&character.id())
            });

        let mut result = Vec::new();

        for character in filtered_list {
            result.push(CharactersAvailableForPurchase::new(
                character.id(),
                character.call_id(),
                character.display_name(),
                character.price(),
            ));
        }

        result
    }
}

#[cfg(test)]
pub mod test {
    use crate::core::game_data::{CharactersAvailableForPurchase, GameWorld, UserInventory};

    use super::GameController;

    #[test]
    pub fn get_character_list_available_for_purchase() {
        let expected = CharactersAvailableForPurchase::new(
            "expected".to_string(),
            "1".to_string(),
            "期待子".to_string(),
            200,
        );
        let expected_not_includes = CharactersAvailableForPurchase::new(
            "test".to_string(),
            "2".to_string(),
            "テスト子".to_string(),
            200,
        );
        let characters = vec![expected.clone(), expected_not_includes.clone()];
        let sut = GameController::new(
            UserInventory::new(Some(vec!["test".to_string()])),
            GameWorld::new(None),
            characters,
        );

        let result = sut.get_character_list_available_for_purchase();

        assert!(result.contains(&expected), "{:?}", result)
    }
}
