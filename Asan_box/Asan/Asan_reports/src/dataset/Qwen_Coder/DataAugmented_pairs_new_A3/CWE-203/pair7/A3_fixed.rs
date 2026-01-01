use std::fmt;

struct Account {
    username: String,
    password: String,
}

struct Authenticator {
    accounts: Vec<Account>,
}

impl Authenticator {
    fn new() -> Self {
        Authenticator {
            accounts: vec![
                Account {
                    username: "alice".to_string(),
                    password: "password123".to_string(),
                },
            ],
        }
    }

    fn verify(&self, user: &str, pass: &str) -> Result<(), AuthError> {
        for acc in &self.accounts {
            if acc.username == user {
                if acc.password == pass {
                    return Ok(());
                } else {
                    return Err(AuthError);
                }
            }
        }
        Err(AuthError)
    }
}

#[derive(Debug)]
struct AuthError;

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid credentials")
    }
}

impl std::error::Error for AuthError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication() {
        let auth = Authenticator::new();
        let result = auth.verify("alice", "wrong_password");
        let error_message = match result {
            Ok(_) => "Login successful".to_string(),
            Err(e) => e.to_string(),
        };
        // This test expects the error message to be generic.
        assert_eq!(error_message, "Invalid credentials", "Authentication error should be generic");
    }
}
