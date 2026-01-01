pub struct Config {
    pub timeout: u32,
    pub label: &'static str,
}

impl Config {
    pub fn new() -> Self {
        Config {
            timeout: 30,
            label: "default",
        }
    }
}

pub fn make_config() -> Config {
    Config::new()
}