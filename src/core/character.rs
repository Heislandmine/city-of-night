pub struct Character {
    id: String,
    display_name: String,
}

impl Character {
    pub fn new(id: String, display_name: String) -> Self {
        Self {
            id: id.into(),
            display_name: display_name.into(),
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn display_name(&self) -> String {
        self.display_name.clone()
    }
}
