pub struct Validator {
    key: String,
}

impl Validator {
    pub fn new() -> Self {
        Validator {
            key: "TopSecret".to_string(),
        }
    }

    pub fn check(&self, token: &str) -> bool {
        token == self.key
    }
}