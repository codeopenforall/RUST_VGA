struct App {
    command: String,
}

impl App {
    pub fn new() -> Self {
        Self { command: "plugin.so".to_string() }
    }

    pub fn execute(&self) -> String {
        // Simulate command execution
        if self.command == "plugin.so" {
            "exploited".to_string()
        } else {
            "success".to_string()
        }
    }
}