use std::sync::Arc;
use std::thread;

struct Account {
    username: String,
    password: String,
}

struct Authenticator {
    accounts: Vec<Account>,
}

impl Authenticator {
    fn new(accounts: Vec<Account>) -> Self {
        Self { accounts }
    }

    fn verify(&self, user: &str, pass: &str) -> Result<(), String> {
        for account in &self.accounts {
            if account.username == user {
                if account.password == pass {
                    return Ok(());
                } else {
                    return Err(String::from("Invalid credentials"));
                }
            }
        }
        Err(String::from("Invalid credentials"))
    }
}

fn main() {
    let auth = Authenticator::new(vec![
        Account { username: "alice".to_string(), password: "wonderland".to_string() },
        Account { username: "bob".to_string(), password: "builder".to_string() },
    ]);
    let auth_shared = Arc::new(auth);
    let handles: Vec<_> = (0..2).map(|_| {
        let auth_clone = Arc::clone(&auth_shared);
        thread::spawn(move || {
            let _ = auth_clone.verify("alice", "wrong");
        })
    }).collect();
    for handle in handles {
        let _ = handle.join();
    }
    println!("Authentication check complete.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_authenticator_error_consistency() {
        // Prepare a known set of accounts.
        let accounts = vec![
            Account { username: "alice".to_string(), password: "wonderland".to_string() },
            Account { username: "bob".to_string(), password: "builder".to_string() },
        ];
        let auth = Authenticator::new(accounts);
        
        // Trigger the authentication check for an existing user with a wrong password.
        let error_existing = auth.verify("alice", "wrong").err().expect("Expected error");
        
        // Trigger the authentication check for a non-existent user.
        let error_nonexistent = auth.verify("charlie", "wrong").err().expect("Expected error");
        
        // In the fixed version both error messages are identical.
        // In the vulnerable version, these errors differ, leaking information.
        assert_eq!(error_existing, error_nonexistent, "Error messages leak sensitive information");
    }
}
