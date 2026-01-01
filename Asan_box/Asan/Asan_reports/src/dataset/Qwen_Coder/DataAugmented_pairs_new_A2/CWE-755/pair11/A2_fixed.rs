use std::fs;
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;

struct Engine {
    config: String,
}

impl Engine {
    fn new() -> Self {
        Engine { config: String::new() }
    }

    fn load_conf(&mut self, path: &str) -> Result<(), io::Error> {
        let content = fs::read_to_string(path)?;
        self.config = content;
        Ok(())
    }
}

fn main() {
    let engine = Arc::new(Mutex::new(Engine::new()));
    let engine_clone = Arc::clone(&engine);
    let handle = thread::spawn(move || {
        let mut eng = engine_clone.lock().unwrap();
        if let Err(e) = eng.load_conf("nonexistent.conf") {
            eprintln!("Error occurred: {}", e);
        }
    });
    handle.join().unwrap();
    let eng = engine.lock().unwrap();
    println!("Configuration loaded: {:?}", eng.config);
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
