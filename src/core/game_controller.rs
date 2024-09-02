use super::{
    actions::{Action, ActionResult, PurchaseCharacterAction},
    call_id::CallId,
    character_sheet::CharacterSheet,
    game_data::{CharactersAvailableForPurchase, GameWorld, UserInventory},
};

pub struct GameController {
    user_inventory: UserInventory,
    world: GameWorld,
    character_sheets: Vec<CharacterSheet>,
    call_ids: Vec<CallId>,
}

impl GameController {
    pub fn new(
        user_inventory: UserInventory,
        world: GameWorld,
        character_sheets: Vec<CharacterSheet>,
        call_ids: Vec<CallId>,
    ) -> Self {
        Self {
            user_inventory,
            world,
            character_sheets,
            call_ids,
        }
    }

    pub fn handle_action(&mut self, action: Action) -> ActionResult {
        match action {
            Action::PurchaseCharacter(id) => {
                let mut command = PurchaseCharacterAction::new(
                    &mut self.world,
                    &mut self.user_inventory,
                    &self.character_sheets,
                );
                command.execute(id)
            }
            _ => ActionResult::none(),
        }
    }

    pub fn get_character_list_available_for_purchase(&self) -> Vec<CharactersAvailableForPurchase> {
        let mut filtered_list = self.character_sheets.iter().filter(|character| {
            !self
                .user_inventory
                .owned_characters()
                .contains(&character.id())
        });

        let mut result = Vec::new();

        for call_id in self.call_ids.iter() {
            let target_character = filtered_list.find(|e| e.id() == call_id.target_item_id());

            match target_character {
                Some(character) => {
                    result.push(CharactersAvailableForPurchase::new(
                        character.id(),
                        call_id.id(),
                        character.display_name(),
                        character.price(),
                    ));
                }
                None => {}
            }
        }

        result
    }
}

#[cfg(test)]
pub mod test {
    use crate::core::{
        call_id::CallId,
        character_sheet::CharacterSheet,
        game_data::{CharactersAvailableForPurchase, GameWorld, UserInventory},
    };

    use super::GameController;

    #[test]
    fn character_list_available_for_purchase_created_from_call_ids_and_sheets() {
        let expected = CharactersAvailableForPurchase::new(
            "expected".to_string(),
            "1".to_string(),
            "期待子".to_string(),
            200,
        );
        let call_ids = vec![CallId::new("1", "expected")];
        let sheets = vec![CharacterSheet::new(
            "expected".to_string(),
            "期待子".to_string(),
            200,
        )];

        let sut = GameController::new(
            UserInventory::new(None),
            GameWorld::new(None),
            sheets,
            call_ids,
        );

        let result = sut.get_character_list_available_for_purchase();

        assert!(result.contains(&expected), "{:?}", result)
    }

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
        let sut = GameController::new(
            UserInventory::new(Some(vec!["test".to_string()])),
            GameWorld::new(None),
            vec![
                CharacterSheet::new("expected".to_string(), "期待子".to_string(), 200),
                CharacterSheet::new("test".to_string(), "テスト子".to_string(), 200),
            ],
            vec![CallId::new("1", "expected"), CallId::new("2", "test")],
        );

        let result = sut.get_character_list_available_for_purchase();

        assert!(result.contains(&expected), "{:?}", result);
        assert!(!result.contains(&expected_not_includes));
    }

    #[cfg(test)]
    pub mod handle_action {
        use crate::core::{
            actions::{Action, ActionResult, TextMessage},
            character_sheet::CharacterSheet,
            game_controller::GameController,
            game_data::{GameWorld, UserInventory},
        };

        #[test]
        pub fn purchase_character() {
            let purchased_character =
                CharacterSheet::new("test-ko".to_string(), "テスト子".to_string(), 200);
            let expected = ActionResult::success(Some(TextMessage::new(
                "テスト子を購入しました".to_string(),
                true,
            )));
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
