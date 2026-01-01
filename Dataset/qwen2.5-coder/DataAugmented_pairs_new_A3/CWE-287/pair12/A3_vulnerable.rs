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
        if let Some(user_token) = input {
            return !user_token.is_empty(); // Vulnerable: Always returns true if input is not empty
        }
        false
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