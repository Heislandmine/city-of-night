use super::character::Character;

pub struct GameData {
    user_inventory: UserInventory,
    game_world: GameWorld,
}

impl GameData {
    pub fn new(user_inventory: UserInventory, game_world: GameWorld) -> Self {
        Self {
            user_inventory,
            game_world,
        }
    }

    pub fn get_characters_available_for_purchase(&self) -> Vec<CharactersAvailableForPurchase> {
        // TODO: 仮実装
        vec![CharactersAvailableForPurchase::new(
            "demo-ko".to_string(),
            "1".to_string(),
            "デモ子".to_string(),
            2000,
        )]
    }

    pub fn user_inventory(&self) -> &UserInventory {
        &self.user_inventory
    }

    pub fn game_world(&self) -> &GameWorld {
        &self.game_world
    }
}

pub struct UserInventory {
    owned_characters: Vec<String>,
}

impl UserInventory {
    pub fn new(init_owned_characters: Option<Vec<String>>) -> Self {
        let owned_characters = match init_owned_characters {
            Some(init) => init,
            None => Vec::new(),
        };

        Self { owned_characters }
    }

    pub fn owned_characters(&self) -> Vec<String> {
        self.owned_characters.clone()
    }

    pub fn add_character(&mut self, character_id: String) {
        self.owned_characters.push(character_id);
    }
}

pub struct GameWorld {
    characters: Vec<Character>,
}

impl GameWorld {
    pub fn new(init_exist_characters: Option<Vec<Character>>) -> Self {
        let init = match init_exist_characters {
            Some(i) => i,
            None => Vec::new(),
        };

        Self { characters: init }
    }

    pub fn add_character(&mut self, character: Character) {
        self.characters.push(character);
    }

    pub fn get_character_by_id(&self, character_id: String) -> Option<&Character> {
        self.characters.iter().find(|e| e.id() == character_id)
    }
}

pub struct CharactersAvailableForPurchase {
    id: String,
    call_id: String,
    display_name: String,
    price: u16,
}

impl CharactersAvailableForPurchase {
    pub fn new(id: String, call_id: String, display_name: String, price: u16) -> Self {
        Self {
            id,
            call_id,
            display_name,
            price,
        }
    }

    pub fn display_name(&self) -> String {
        self.display_name.clone()
    }

    pub fn call_id(&self) -> String {
        self.call_id.clone()
    }
}
#[cfg(test)]
mod game_data_test {
    use crate::core::character::Character;

    fn create_test_character() -> Character {
        Character::new("test".to_string(), "test".to_string())
    }

    #[cfg(test)]
    mod user_inventory_test {
        use crate::core::game_data::{game_data_test::create_test_character, UserInventory};

        #[test]
        fn test_user_inventory_new() {
            let sut = UserInventory::new(Some(vec![String::from("test")]));
            assert_eq!(sut.owned_characters(), vec![String::from("test")])
        }

        #[test]
        fn test_user_inventory_new_with_init_vec_none() {
            let sut = UserInventory::new(None);
            assert_eq!(sut.owned_characters().len(), 0);
        }

        #[test]
        fn add_character_test() {
            let mut sut = UserInventory::new(None);
            let character = create_test_character();

            sut.add_character(character.id());

            assert_eq!(sut.owned_characters()[0], character.id())
        }
    }

    #[cfg(test)]
    mod game_world_test {
        use crate::core::game_data::GameWorld;

        use super::create_test_character;

        #[test]
        fn add_character() {
            let character = create_test_character();
            let mut sut = GameWorld::new(None);
            let character_id = character.id();
            sut.add_character(character);

            match sut.get_character_by_id(character_id.clone()) {
                Some(e) => assert_eq!(e.id(), character_id),
                None => assert!(false),
            }
        }
    }
}
