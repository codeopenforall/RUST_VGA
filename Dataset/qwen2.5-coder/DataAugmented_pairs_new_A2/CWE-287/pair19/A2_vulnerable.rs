use std::sync::Arc;
use std::thread;

struct Authenticator {
    secret: &'static str,
}

impl Authenticator {
    fn new() -> Self {
        Authenticator { secret: "TopSecret" }
    }

    fn validate(&self, input: &str) -> bool {
        if input.len() < 3 || self.secret.len() < 3 {
            return false;
        }
        unsafe {
            let input_ptr = input.as_bytes().as_ptr();
            let secret_ptr = self.secret.as_bytes().as_ptr();
            std::slice::from_raw_parts(input_ptr, 3)
                == std::slice::from_raw_parts(secret_ptr, 3)
        }
    }
}

fn main() {
    let authenticator = Arc::new(Authenticator::new());
    let tokens = vec!["TopBanana", "TopSecret", "BadToken", "TopCow"];
    let threads: Vec<_> = tokens.into_iter().map(|token| {
        let auth = Arc::clone(&authenticator);
        let tok = token.to_string();
        thread::spawn(move || {
            let is_valid = auth.validate(&tok);
            println!("Token: {}, Validated: {}", tok, is_valid);
        })
    }).collect();
    for thread in threads {
        thread.join().unwrap();
    }
}