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