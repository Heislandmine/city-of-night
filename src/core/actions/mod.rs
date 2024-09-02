use super::{
    character_sheet::CharacterSheet,
    factories::create_character,
    game_data::{GameWorld, UserInventory},
    mode::ViewsMode,
};

pub enum KeyPressedEvent {
    None,
    KeyPressed(char),
    PressedBackspace,
    PressedEnter(String),
}
pub enum Action {
    None,
    PopOneCharacterFromInputtedString,
    PushToCharacterFromInputtedString(char),
    PurchaseCharacter(String),
    Quit,
    Navigate(ViewsMode),
    ResetMessage,
}

#[derive(Debug, PartialEq)]
pub enum ActionStatus {
    Success,
    Failed,
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TextMessage {
    pub content: String,
    pub wait_enter_pressed: bool,
}

impl TextMessage {
    pub fn new(content: String, wait_enter_pressed: bool) -> Self {
        Self {
            content,
            wait_enter_pressed,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ActionResult {
    pub status: ActionStatus,
    pub message: Option<TextMessage>,
}

impl ActionResult {
    pub fn new(status: ActionStatus, message: Option<TextMessage>) -> Self {
        Self { status, message }
    }

    pub fn success(message: Option<TextMessage>) -> Self {
        ActionResult::new(ActionStatus::Success, message)
    }

    pub fn none() -> Self {
        ActionResult::new(ActionStatus::None, None)
    }
}

pub struct PurchaseCharacterAction<'a> {
    game_world: &'a mut GameWorld,
    user_inventory: &'a mut UserInventory,
    character_sheets: &'a Vec<CharacterSheet>,
}

impl<'a> PurchaseCharacterAction<'a> {
    pub fn new(
        game_world: &'a mut GameWorld,
        user_inventory: &'a mut UserInventory,
        character_sheets: &'a Vec<CharacterSheet>,
    ) -> Self {
        Self {
            game_world,
            user_inventory,
            character_sheets,
        }
    }

    pub fn execute(&mut self, character_id: String) -> ActionResult {
        let target_sheet = self
            .character_sheets
            .iter()
            .find(|e| e.id() == character_id);

        match target_sheet {
            Some(sheet) => {
                let created_character = create_character(sheet.clone());
                let message = format!("{}を購入しました", created_character.display_name());
                self.game_world.add_character(created_character);
                self.user_inventory.add_character(character_id.clone());

                ActionResult::success(Some(TextMessage::new(message, true)))
            }
            None => ActionResult::new(
                ActionStatus::Failed,
                Some(TextMessage::new(
                    String::from(format!(
                        "id:{}を持つキャラクターシートがありません。",
                        character_id
                    )),
                    true,
                )),
            ),
        }
    }
}

#[cfg(test)]
pub mod actions_test {
    #[cfg(test)]
    pub mod purchase_character_action {
        use rstest::rstest;

        use crate::core::{
            actions::{ActionResult, PurchaseCharacterAction, TextMessage},
            character_sheet::CharacterSheet,
            game_data::{GameWorld, UserInventory},
        };

        #[rstest]
        #[case("test-ko".to_string(), ActionResult::success(Some(TextMessage::new(
            "テスト子を購入しました".to_string(),
            true,
        ))))]
        #[case("test-ko-2".to_string(), ActionResult::success(Some(TextMessage::new(
            "テスト2子を購入しました".to_string(),
            true,
        ))))]
        fn execute(#[case] character_id: String, #[case] expected: ActionResult) {
            let sheets = vec![
                CharacterSheet::new("test-ko".to_string(), "テスト子".to_string(), 200, 1000),
                CharacterSheet::new("test-ko-2".to_string(), "テスト2子".to_string(), 200, 1000),
            ];

            let mut game_world = GameWorld::new(None);
            let mut user_inventory = UserInventory::new(None);
            let mut sut =
                PurchaseCharacterAction::new(&mut game_world, &mut user_inventory, &sheets);
            let result = sut.execute(character_id.clone());

            assert_eq!(result, expected);

            // キャラクター生成の確認
            match game_world.get_character_by_id(character_id.clone()) {
                Some(e) => assert_eq!(e.id(), character_id),
                None => assert!(false),
            }

            // ユーザーインベントリーにキャラクターIDが追加されたか確認
            assert!(user_inventory.owned_characters().contains(&character_id))
        }
    }
}
