use std::sync::{Arc, Mutex};
use std::fmt;
#[derive(Debug)]
struct Account {
    username: String,
    password: String,
}
#[derive(Debug)]
struct AuthError;
impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid credentials")
    }
}
struct Authenticator {
    accounts: Arc<Mutex<Vec<Account>>>,
}
impl Authenticator {
    fn new() -> Self {
        let accounts = vec![
            Account {
                username: "alice".to_string(),
                password: "secret".to_string(),
            },
            Account {
                username: "bob".to_string(),
                password: "hunter2".to_string(),
            },
        ];
        Authenticator { accounts: Arc::new(Mutex::new(accounts)) }
    }
    fn verify(&self, user: &str, pass: &str) -> Result<(), AuthError> {
        let accounts = self.accounts.lock().unwrap();
        for acc in accounts.iter() {
            if acc.username == user && acc.password == pass {
                return Ok(());
            }
        }
        Err(AuthError)
    }
}
fn main() {
    let auth = Authenticator::new();
    let username = "alice";
    let password = "wrong_password";
    match auth.verify(username, password) {
        Ok(_) => println!("Login successful"),
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

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
