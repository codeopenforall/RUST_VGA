use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Resolver;
impl Resolver {
    pub fn resolve(&self, input: &str) -> Result<String, String> {
        let resolved = String::from(input);
        Ok(resolved)
    }
}

fn is_within_safe_directory(path: &Path, safe_dir: &Path) -> bool {
    path.starts_with(safe_dir)
}

fn process(input: &str) -> Result<String, String> {
    let resolver = Resolver;
    let resolved_path = resolver.resolve(input)?;
    let safe_dir = Path::new("safe");

    if !is_within_safe_directory(Path::new(&resolved_path), safe_dir) {
        return Err("Attempted to access a path outside the safe directory".to_string());
    }

    let file = File::open(&resolved_path)
        .map_err(|e| format!("Error opening file: {}", e))?;
    let mut buf_reader = std::io::BufReader::new(file);
    let mut content = String::new();
    buf_reader
        .read_to_string(&mut content)
        .map_err(|e| format!("Error reading file: {}", e))?;
    Ok(content)
}

fn main() {
    let shared_result = Arc::new(Mutex::new(String::new()));
    let shared_result_clone = Arc::clone(&shared_result);
    let args: Vec<String> = std::env::args().collect();
    let input_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "test_symlink.txt".to_string()
    };
    let handle = thread::spawn(move || {
        match process(&input_path) {
            Ok(content) => {
                let mut data = shared_result_clone.lock().unwrap();
                *data = content;
            }
            Err(e) => {
                panic!("Operation failed: {}", e);
            }
        }
    });
    handle.join().unwrap();
    let result = shared_result.lock().unwrap();
    println!("File content: {}", *result);
}