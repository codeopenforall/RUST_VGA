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