struct Authenticator {
    valid: String,
}

impl Authenticator {
    pub fn check(&self, token: &str) -> bool {
        token == self.valid
    }
}

fn main() {
    let auth = Authenticator {
        valid: "supersecrettoken".to_string(),
    };
    println!("Authentication result: {}", auth.check("supersecrettoken"));
}