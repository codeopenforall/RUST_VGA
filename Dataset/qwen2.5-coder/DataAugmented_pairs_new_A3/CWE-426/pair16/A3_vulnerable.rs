struct Loader {
    command: String,
}

impl Loader {
    pub fn new() -> Self {
        Loader {
            command: "example_app".to_string(),
        }
    }

    pub fn is_secure(&self) -> bool {
        self.command.starts_with("/")
    }
}