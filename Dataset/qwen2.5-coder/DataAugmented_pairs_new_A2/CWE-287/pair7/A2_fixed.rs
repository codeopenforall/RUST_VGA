use std::sync::Arc;
use std::thread;

struct Authenticator {
    token: String,
}

impl Authenticator {
    fn new(token: &str) -> Authenticator {
        Authenticator {
            token: token.to_string(),
        }
    }

    fn verify(&self, input: &str) -> bool {
        self.token == input
    }
}

fn main() {
    let auth = Authenticator::new("super_secret");
    let auth_arc = Arc::new(auth);
    let threads: Vec<_> = (0..4)
        .map(|i| {
            let auth_ref = Arc::clone(&auth_arc);
            thread::spawn(move || {
                if auth_ref.verify("super_secret") {
                    println!("Thread {}: Access granted", i);
                } else {
                    println!("Thread {}: Access denied", i);
                }
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
}