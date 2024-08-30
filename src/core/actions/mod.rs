use super::{
    character::Character,
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

#[derive(Debug, PartialEq)]
pub struct ActionResult {
    pub status: ActionStatus,
    pub message: Option<String>,
}

impl ActionResult {
    pub fn new(status: ActionStatus, message: Option<String>) -> Self {
        Self { status, message }
    }
}

pub struct PurchaseCharacterAction<'a> {
    game_world: &'a mut GameWorld,
    user_inventory: &'a mut UserInventory,
}

impl<'a> PurchaseCharacterAction<'a> {
    pub fn new(game_world: &'a mut GameWorld, user_inventory: &'a mut UserInventory) -> Self {
        Self {
            game_world,
            user_inventory,
        }
    }

    pub fn execute(&mut self, character_id: String) -> ActionResult {
        let created_character = Character::new(character_id.clone(), "テスト子".to_string());
        let message = format!("{}を購入しました", created_character.display_name());
        self.game_world.add_character(created_character);
        self.user_inventory.add_character(character_id.clone());

        ActionResult::new(ActionStatus::Success, Some(message))
    }
}

#[cfg(test)]
pub mod actions_test {
    #[cfg(test)]
    pub mod purchase_character_action {
        use crate::core::{
            actions::{ActionResult, ActionStatus, PurchaseCharacterAction},
            game_data::{GameWorld, UserInventory},
        };

        #[test]
        fn execute() {
            let mut game_world = GameWorld::new(None);
            let mut user_inventory = UserInventory::new(None);
            let character_id = "test-ko".to_string();
            let mut sut = PurchaseCharacterAction::new(&mut game_world, &mut user_inventory);
            let expected = ActionResult::new(
                ActionStatus::Success,
                Some("テスト子を購入しました".to_string()),
            );
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
