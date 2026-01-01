use std::collections::HashMap;

#[derive(Debug)]
enum ErrorType {
    InvalidInput,
}

#[derive(Debug)]
struct App {
    users: HashMap<String, String>,
}

impl App {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        users.insert("alice".to_string(), "password123".to_string());
        App { users }
    }

    pub fn verify(&self, account: &str, secret: &str) -> Result<(), ErrorType> {
        let is_valid = if let Some(valid_secret) = self.users.get(account) {
            valid_secret == secret
        } else {
            false
        };

        if is_valid {
            Ok(())
        } else {
            Err(ErrorType::InvalidInput)
        }
    }
}