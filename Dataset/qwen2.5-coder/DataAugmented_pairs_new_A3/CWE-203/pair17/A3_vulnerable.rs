use std::collections::HashMap;

#[derive(Debug)]
enum ErrorType {
    UserMissing,
    CredentialMismatch,
}

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
        if let Some(valid_secret) = self.users.get(account) {
            if valid_secret == secret {
                return Ok(());
            } else {
                return Err(ErrorType::CredentialMismatch);
            }
        }
        Err(ErrorType::UserMissing)
    }
}