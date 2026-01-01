struct Authenticator {
    key: String,
}

impl Authenticator {
    fn new() -> Self {
        Authenticator {
            key: "valid_token".to_string(),
        }
    }

    pub fn check(&self, credential: &str) -> Result<(), &str> {
        if credential == self.key {
            Ok(())
        } else {
            Err("Not authorized")
        }
    }
}

fn main() {
    let auth = Authenticator::new();
    match auth.check("valid_token") {
        Ok(_) => println!("Access granted (fixed) for input: valid_token"),
        Err(e) => println!("Access denied (fixed): {}", e),
    }

    match auth.check("invalid_token") {
        Ok(_) => println!("Access granted (fixed) for input: invalid_token"),
        Err(e) => println!("Access denied (fixed): {}", e),
    }
}

fn test_authentication() {
    // The test oracle calls the credential verification function.
    // For a secure implementation, an invalid credential must be rejected.
    let auth = Authenticator::new();
    let result = auth.check("invalid_token");
    // In a secure implementation, the check should return an error.
    assert!(result.is_err(), "Authentication should fail for an invalid credential.");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oracle() {
        test_authentication();
    }
}
