use super::{character::Character, character_sheet::CharacterSheet};

pub fn create_character(from: CharacterSheet) -> Character {
    Character::new(from.id(), from.display_name(), from.base_hp())
}

#[cfg(test)]
mod test_character_factory {
    use rstest::rstest;

    use crate::core::{
        character::Character, character_sheet::CharacterSheet, factories::create_character,
    };

    #[rstest]
    #[case(CharacterSheet::new("test-ko".to_string(), "テスト子".to_string(), 200, 1200), Character::new("test-ko".to_string(), "テスト子".to_string(), 1200))]
    #[case(CharacterSheet::new("test-ko-2".to_string(), "テスト2子".to_string(), 200, 1500), Character::new("test-ko-2".to_string(), "テスト2子".to_string(), 1500))]
    fn character_creation_from_character_sheet(
        #[case] data: CharacterSheet,
        #[case] expected: Character,
    ) {
        let result = create_character(data);

        assert_eq!(result, expected);
    }
}
