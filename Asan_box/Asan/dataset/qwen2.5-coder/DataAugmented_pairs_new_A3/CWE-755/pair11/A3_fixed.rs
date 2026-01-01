use std::fs;
use std::io;

struct Engine {
    config: String,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            config: String::new(),
        }
    }

    pub fn load_conf(&mut self, path: &str) -> io::Result<()> {
        let content = fs::read_to_string(path)?;
        self.config = content;
        Ok(())
    }
}