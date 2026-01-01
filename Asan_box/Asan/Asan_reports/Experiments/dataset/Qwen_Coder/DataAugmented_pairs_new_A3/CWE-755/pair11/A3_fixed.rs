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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // This test verifies that attempting to load a configuration from a nonexistent file results in an error.
    // The fixed implementation should return an Err variant, while the vulnerable one erroneously returns Ok.
    #[test]
    fn test_missing_file_error() {
        let engine = Arc::new(Mutex::new(Engine::new()));
        {
            let mut eng = engine.lock().unwrap();
            let res = eng.load_conf("nonexistent.conf");
            assert!(
                res.is_err(),
                "Expected an error when loading a nonexistent configuration file."
            );
        }
    }
}
