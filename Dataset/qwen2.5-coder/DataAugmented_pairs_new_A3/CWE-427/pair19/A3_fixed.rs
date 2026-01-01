struct Application;

impl Application {
    pub fn resolve_path(&self) -> String {
        let custom = std::env::var("MALICIOUS_PATH").unwrap_or_default();
        let mut chosen_path = String::new();

        if !custom.is_empty() && custom.starts_with("/usr/bin") {
            if let Ok(valid) = std::str::from_utf8(custom.as_bytes()) {
                chosen_path = valid.to_string();
            }
        }

        if chosen_path.is_empty() {
            chosen_path = String::from("/usr/bin:/bin");
        }

        chosen_path
    }
}