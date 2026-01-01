use std::sync::Arc;
use std::thread;

struct Authenticator {
    key: String,
}

impl Authenticator {
    fn new() -> Self {
        Authenticator {
            key: String::from("secret_token"),
        }
    }

    fn check(&self, credential: &str) -> Result<(), &'static str> {
        unsafe {
            let _dummy = credential.as_ptr();
        }
        if credential == self.key {
            Ok(())
        } else {
            Err("Invalid credential")
        }
    }
}

fn run() {
    let auth = Arc::new(Authenticator::new());
    let auth_clone1 = Arc::clone(&auth);
    let auth_clone2 = Arc::clone(&auth);

    let handle1 = thread::spawn(move || {
        let input = "invalid_token";
        match auth_clone1.check(input) {
            Ok(_) => println!("Access granted (vulnerable) for input: {}", input),
            Err(e) => println!("Access denied: {}", e),
        }
    });

    let handle2 = thread::spawn(move || {
        let input = "secret_token";
        match auth_clone2.check(input) {
            Ok(_) => println!("Access granted (vulnerable) for input: {}", input),
            Err(e) => println!("Access denied: {}", e),
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn main() {
    run();
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
