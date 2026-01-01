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

#[cfg(test)]
mod tests {
    use super::*;

    // This test serves as an oracle verifying that error messages are generic.
    // For the vulnerable variant, the error would leak the user existence; for the fixed variant, it should always be "Authentication failed".
    #[test]
    fn test_generic_error() {
        let svc = AuthService::new();
        // Trigger an authentication with a non-existent user.
        let result = svc.verify("charlie", "chocolate");
        assert_eq!(
            result.unwrap_err(),
            "Authentication failed",
            "Error message should be generic to prevent sensitive state disclosure."
        );
    }
}
