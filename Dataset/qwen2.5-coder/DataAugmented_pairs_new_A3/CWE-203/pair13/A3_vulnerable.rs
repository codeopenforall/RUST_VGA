struct Auth;

impl Auth {
    fn new() -> Self {
        Auth
    }

    fn login(&self, username: &str, password: &str) -> Result<(), String> {
        if username == "alice" {
            if password == "correct_password" {
                Ok(())
            } else {
                Err("password incorrect".to_string())
            }
        } else {
            Err("unknown user".to_string())
        }
    }
}