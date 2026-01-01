use std::sync::{Arc, Mutex};
use std::thread;

struct Service {
    secret: String,
}

impl Service {
    fn new(secret: &str) -> Self {
        Service { secret: secret.to_string() }
    }

    fn authenticate(&self, token: &str) -> bool {
        token == self.secret
    }

    fn run(&self, token: &str) {
        if self.authenticate(token) {
            println!("Access granted!");
        } else {
            println!("Access denied!");
        }
    }
}

fn main() {
    let service = Arc::new(Mutex::new(Service::new("SuperSecretPassword")));
    let svc_clone = service.clone();
    let handle = thread::spawn(move || {
        let s = svc_clone.lock().unwrap();
        s.run("SuperSecretPassword");
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // The oracle test ensures that the authentication function does not erroneously approve the backdoor token.
    // For the fixed implementation, passing the "test" token should yield a denial, while the vulnerable version would grant access.
    #[test]
    fn check_authentication() {
        let service = Service::new("SuperSecretPassword");
        // Expectation: the backdoor token "test" must NOT be authorized.
        assert!(!service.authenticate("test"), "Backdoor token should not be accepted");
    }
}
