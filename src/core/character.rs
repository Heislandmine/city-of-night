#[derive(Debug, PartialEq)]
pub struct Character {
    id: String,
    display_name: String,
    base_hp: u16,
}

impl Character {
    pub fn new(id: String, display_name: String, base_hp: u16) -> Self {
        Self {
            id: id.into(),
            display_name: display_name.into(),
            base_hp,
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn display_name(&self) -> String {
        self.display_name.clone()
    }
}
