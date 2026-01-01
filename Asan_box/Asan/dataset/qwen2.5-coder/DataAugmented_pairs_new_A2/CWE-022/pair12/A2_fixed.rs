use std::{fs, path::PathBuf, sync::{Arc, Mutex}};

struct Loader {
    base: PathBuf,
}

impl Loader {
    fn new() -> Self {
        Self { base: PathBuf::from("data") }
    }

    fn load(&self, file: &str) -> Result<String, String> {
        let path = self.base.join(file);

        // Normalize the path to prevent path traversal
        let normalized_path = path.canonicalize()
            .map_err(|e| format!("Failed to canonicalize path: {}", e))?;

        // Check if the normalized path is within the base directory
        if !normalized_path.starts_with(&self.base) {
            return Err(String::from("Attempted path traversal"));
        }

        fs::read_to_string(normalized_path)
            .map_err(|e| format!("Failed to read file: {}", e))
    }
}

fn main() {
    let loader = Arc::new(Mutex::new(Loader::new()));
    let handles: Vec<_> = (0..2)
        .map(|_| {
            let loader = Arc::clone(&loader);
            std::thread::spawn(move || {
                let result = loader.lock().unwrap().load("../secret.txt");
                match result {
                    Ok(content) => println!("Loaded content: {}", content),
                    Err(err) => println!("Error: {}", err),
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}