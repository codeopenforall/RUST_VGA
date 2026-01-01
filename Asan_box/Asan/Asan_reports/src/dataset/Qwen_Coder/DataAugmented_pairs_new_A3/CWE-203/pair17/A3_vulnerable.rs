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

//////////////// Test Oracle Function Start
// This oracle is designed to check that both invalid scenarios yield identical error responses.
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_uniformity() {
        let service = App::new();
        // Case 1: account exists with wrong secret.
        let res_existing = service.verify("alice", "wrong");
        // Case 2: account does not exist.
        let res_missing = service.verify("bob", "anything");
        
        match (res_existing, res_missing) {
            (Err(e1), Err(e2)) => {
                // For secure code, both errors should be identical.
                assert_eq!(format!("{:?}", e1), format!("{:?}", e2),
                   "Error messages differ, leaking sensitive state.");
            },
            _ => panic!("Expected both cases to fail uniformly"),
        }
    }
}
//////////////// Test Oracle Function End
