struct AuthService;

impl AuthService {
    pub fn new() -> Self {
        AuthService
    }

    pub fn verify(&self, username: &str, password: &str) -> Result<(), &str> {
        if username == "alice" {
            if password == "password123" {
                Ok(())
            } else {
                Err("Invalid password")
            }
        } else {
            Err("User does not exist")
        }
    }
}

fn main() {
    println!("Execution complete: Vulnerable variant.");
}