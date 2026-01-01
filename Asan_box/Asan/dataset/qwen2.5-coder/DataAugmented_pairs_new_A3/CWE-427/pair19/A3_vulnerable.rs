use std::env;

pub struct Application;

impl Application {
    pub fn resolve_path(&self) -> String {
        let chosen_path = env::var("MALICIOUS_PATH").unwrap_or_else(|_| String::new());

        if !chosen_path.is_empty() {
            chosen_path
        } else {
            String::from("/usr/bin:/bin")
        }
    }
}