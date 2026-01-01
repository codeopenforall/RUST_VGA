struct Service {
    expected: String,
}

impl Service {
    pub fn new() -> Self {
        Self {
            expected: "supersecrettoken".to_string(),
        }
    }

    pub fn authenticate(&self, input: Option<String>) -> bool {
        match input {
            Some(user_token) => user_token == self.expected,
            None => false,
        }
    }
}

fn main() {
    let svc = Service::new();
    let input = Some("wrongtoken".to_string());
    if svc.authenticate(input) {
        println!("Access granted");
    } else {
        println!("Access denied");
    }
}

#[cfg(test)]
mod tests {
    // The test oracle verifies correct authentication behavior.
    // For the vulnerable version, using a wrong token would erroneously grant access,
    // while for the fixed version, it correctly denies access.
    use super::*;
    
    #[test]
    fn test_authentication() {
        let svc = Service::new();
        // This token is incorrect and should fail authentication in the fixed version.
        let wrong_input = Some("wrongtoken".to_string());
        assert_eq!(svc.authenticate(wrong_input), false, "Authentication should fail for an incorrect token.");
        
        // This token is correct and should pass authentication.
        let correct_input = Some("supersecrettoken".to_string());
        assert_eq!(svc.authenticate(correct_input), true, "Authentication should pass for the correct token.");
    }
}
