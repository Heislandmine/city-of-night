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

#[cfg(test)]
mod game_data_test {
    use crate::core::character::Character;

    use super::UserInventory;

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
        let character = Character::new("test".to_string(), "test".to_string());

        sut.add_character(character.id());

        assert_eq!(sut.owned_characters()[0], character.id())
    }
}
