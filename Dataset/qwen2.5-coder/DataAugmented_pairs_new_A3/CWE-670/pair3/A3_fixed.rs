struct UserManager {
    access: bool,
}

impl UserManager {
    pub fn new() -> Self {
        UserManager { access: false }
    }

    pub fn evaluate(&mut self, user: &str) {
        match user {
            "admin" => {
                self.access = true;
            },
            "guest" => {
                self.access = false;
            },
            _ => {
                self.access = false;
            },
        }
    }

    pub fn is_allowed(&self) -> bool {
        self.access
    }
}