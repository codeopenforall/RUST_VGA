use std::env;
use std::sync::Arc;
use std::thread;

pub fn verify(token: &str) -> bool {
    unsafe {
        let expected = "secret";
        if token.len() < expected.len() {
            return false;
        }
        let token_ptr = token.as_ptr();
        let expected_ptr = expected.as_ptr();
        for i in 0..expected.len() {
            if *token_ptr.add(i) != *expected_ptr.add(i) {
                return false;
            }
        }
        true
    }
}

fn process(auth: Arc<String>) {
    let result = verify(&auth);
    if result {
        println!("Access Granted");
    } else {
        println!("Access Denied");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let token_input = if args.len() > 1 {
        args[1].clone()
    } else {
        String::new()
    };
    let token_arc = Arc::new(token_input);
    let mut handles = vec![];
    for _ in 0..4 {
        let token_clone = Arc::clone(&token_arc);
        handles.push(thread::spawn(move || process(token_clone)));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

//////////////////////////////////////////////////////////////
// Test oracle function for verifying authentication behavior.
// This test uses an invalid token that begins with "sec" but is not equal to "secret".
// In the flawed implementation, the token is erroneously accepted (failing the test).
// In the corrected implementation, the token is correctly rejected (passing the test).
// This function should be placed in a separate test module.
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::sync::Arc;
    // Assume that the "verify" function is imported from the library.
    use super::verify;

    #[test]
    fn test_bad_token() {
        let bad_token = "secXXX"; // starts with "sec" but is not the full credential.
        // The fixed version should reject this token.
        assert!(!verify(bad_token), "Authentication incorrectly succeeded for invalid token");
    }
}
