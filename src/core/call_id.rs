pub struct CallId {
    call_id: String,
    target_item_id: String,
}

impl CallId {
    pub fn new<T, U>(call_id: T, target_item_id: U) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        Self {
            call_id: call_id.into(),
            target_item_id: target_item_id.into(),
        }
    }

    pub fn id(&self) -> String {
        self.call_id.clone()
    }

    pub fn target_item_id(&self) -> String {
        self.target_item_id.clone()
    }
}
