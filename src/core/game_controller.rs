use super::{
    actions::{Action, ActionResult, ActionStatus, PurchaseCharacterAction},
    call_id::CallId,
    character::Character,
    character_sheet::CharacterSheet,
    game_data::{CharactersAvailableForPurchase, GameWorld, UserInventory},
};

pub struct GameController {
    user_inventory: UserInventory,
    world: GameWorld,
    character_sheets: Vec<CharacterSheet>,
    purchase_character_call_ids: Vec<CallId>,
    current_breaking_character: Option<String>,
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
            purchase_character_call_ids: call_ids,
            current_breaking_character: None,
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
                let result = command.execute(id.clone());

                if result.status == ActionStatus::Success
                    && self.current_breaking_character.is_none()
                {
                    self.current_breaking_character = Some(id.clone())
                }

                result
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

        for call_id in self.purchase_character_call_ids.iter() {
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

    pub fn change_current_breaking_character<T>(&mut self, id: Option<T>)
    where
        T: Into<String>,
    {
        match id {
            Some(s) => self.current_breaking_character = Some(s.into()),
            None => self.current_breaking_character = None,
        }
    }

    pub fn get_current_breaking_character(&self) -> Option<Character> {
        match &self.current_breaking_character {
            Some(id) => self.world.get_character_by_id(id.clone()),
            None => None,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::core::{
        call_id::CallId,
        character::Character,
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
            1000,
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
                CharacterSheet::new("expected".to_string(), "期待子".to_string(), 200, 1000),
                CharacterSheet::new("test".to_string(), "テスト子".to_string(), 200, 1000),
            ],
            vec![CallId::new("1", "expected"), CallId::new("2", "test")],
        );

        let result = sut.get_character_list_available_for_purchase();

        assert!(result.contains(&expected), "{:?}", result);
        assert!(!result.contains(&expected_not_includes));
    }

    #[test]
    fn set_currently_breaking_character_id() {
        let expected_id = "test-ko".to_string();
        let expected_character =
            Character::new("test-ko".to_string(), "テスト子".to_string(), 1000);
        let mut sut = GameController::new(
            UserInventory::new(None),
            GameWorld::new(Some(vec![expected_character.clone()])),
            Vec::new(),
            Vec::new(),
        );

        sut.change_current_breaking_character(Some(expected_id));

        let result = sut.get_current_breaking_character();

        match result {
            Some(e) => assert_eq!(e, expected_character),
            None => assert!(false),
        }
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
                CharacterSheet::new("test-ko".to_string(), "テスト子".to_string(), 200, 1000);
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

            match sut.current_breaking_character {
                Some(e) => assert_eq!(e, purchased_character.id()),
                None => assert!(false),
            }
        }
    }
}
