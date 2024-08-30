use super::{
    actions::{Action, ActionResult, ActionStatus, PurchaseCharacterAction},
    character_sheet::CharacterSheet,
    game_data::{CharactersAvailableForPurchase, GameWorld, UserInventory},
};

pub struct GameController {
    user_inventory: UserInventory,
    world: GameWorld,
    character_sheets: Vec<CharacterSheet>,
    character_list_available_for_purchase: Vec<CharactersAvailableForPurchase>,
}

impl GameController {
    pub fn new(
        user_inventory: UserInventory,
        world: GameWorld,
        character_sheets: Vec<CharacterSheet>,
        character_list_available_for_purchase: Vec<CharactersAvailableForPurchase>,
    ) -> Self {
        Self {
            user_inventory,
            world,
            character_sheets,
            character_list_available_for_purchase,
        }
    }

    pub fn handle_action(&mut self, action: Action) -> ActionResult {
        match action {
            Action::PurchaseCharacter(id) => {
                let mut command =
                    PurchaseCharacterAction::new(&mut self.world, &mut self.user_inventory);
                command.execute(id);

                ActionResult::new(ActionStatus::Success, None)
            }
            Action::None => ActionResult::new(ActionStatus::None, None),
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
            Vec::new(),
            characters,
        );

        let result = sut.get_character_list_available_for_purchase();

        assert!(result.contains(&expected), "{:?}", result)
    }

    #[cfg(test)]
    pub mod handle_action {
        use crate::core::{
            actions::{Action, ActionResult, ActionStatus},
            character_sheet::CharacterSheet,
            game_controller::GameController,
            game_data::{GameWorld, UserInventory},
        };

        #[test]
        pub fn purchase_character() {
            let purchased_character =
                CharacterSheet::new("test-ko".to_string(), "テスト子".to_string(), 200);
            let expected = ActionResult::new(ActionStatus::Success, None);
            let character_sheets = vec![purchased_character.clone()];
            let mut sut = GameController::new(
                UserInventory::new(None),
                GameWorld::new(None),
                character_sheets,
                Vec::new(),
            );
            let action = Action::PurchaseCharacter(purchased_character.id());

            let result = sut.handle_action(action);
            assert_eq!(result, expected);
            assert!(sut
                .user_inventory
                .is_character_owned(purchased_character.id()));
            assert!(sut.world.is_character_exist(purchased_character.id()))
        }
    }
}
