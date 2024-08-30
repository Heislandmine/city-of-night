#[derive(Clone)]
pub struct CharacterSheet {
    id: String,
    display_name: String,
    price: u16,
}

impl CharacterSheet {
    pub fn new(id: String, display_name: String, price: u16) -> Self {
        Self {
            id,
            display_name: display_name,
            price,
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn display_name(&self) -> String {
        self.display_name.clone()
    }

    pub fn price(&self) -> u16 {
        self.price
    }
}
