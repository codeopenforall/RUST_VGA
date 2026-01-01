struct Config {
    threshold: u32,
    message: String,
}

impl Config {
    pub fn validate(&self) -> bool {
        self.threshold > 0 && !self.message.is_empty()
    }
}

pub fn load_configuration() -> Config {
    Config {
        threshold: 100,
        message: "default".to_string(),
    }
}