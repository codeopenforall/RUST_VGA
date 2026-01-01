use std::fmt;

#[derive(Debug)]
enum AuthError {
    UserNotFound(String),
    WrongPassword(String),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::UserNotFound(msg) => write!(f, "{}", msg),
            AuthError::WrongPassword(msg) => write!(f, "{}", msg),
        }
    }
}

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

    pub fn verify(&self, user: &str, pass: &str) -> Result<(), AuthError> {
        for acc in &self.accounts {
            if acc.username == user {
                if acc.password == pass {
                    return Ok(());
                } else {
                    return Err(AuthError::WrongPassword(format!(
                        "Account {} exists but password is incorrect", user
                    )));
                }
            }
        }
        Err(AuthError::UserNotFound(format!("User {} does not exist", user)))
    }
}

fn main() {
    let auth = Authenticator::new();
    match auth.verify("alice", "wrong_password") {
        Ok(_) => println!("Login successful"),
        Err(e) => {
            match e {
                AuthError::UserNotFound(_) => std::process::exit(1),
                AuthError::WrongPassword(_) => std::process::exit(2),
            }
        }
    }
}