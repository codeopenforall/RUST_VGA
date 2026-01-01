struct Auth;

impl Auth {
    fn new() -> Self {
        Auth
    }

    fn login(&self, username: &str, password: &str) -> Result<(), String> {
        // Simulate authentication logic
        if username == "alice" && password == "correct_password" {
            Ok(())
        } else {
            Err("invalid credentials".to_string())
        }
    }
}