struct Config {
    secret: String,
}

impl Config {
    pub fn process(&self, success: bool) -> Result<(), String> {
        if success {
            Ok(())
        } else {
            return Err("Error during processing".to_string());
        }
    }
}