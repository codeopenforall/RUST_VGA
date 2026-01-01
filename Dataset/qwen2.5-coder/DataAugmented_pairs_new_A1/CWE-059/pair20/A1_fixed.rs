use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;

struct PathResolver {
    path: String,
}

impl PathResolver {
    fn new(input: &str) -> Self {
        Self { path: input.to_owned() }
    }

    fn is_link(&self) -> bool {
        if let Ok(metadata) = fs::symlink_metadata(&self.path) {
            metadata.file_type().is_symlink()
        } else {
            false
        }
    }

    fn resolve(&self) -> PathBuf {
        if self.is_link() {
            // Avoid using unsafe code
            if let Ok(target) = fs::read_link(&self.path) {
                return target;
            }
        }
        PathBuf::from(&self.path)
    }
}

fn process_file(input: &str) -> io::Result<String> {
    let resolver = PathResolver::new(input);
    let resolved_path = resolver.resolve();

    // Check if the resolved path is a symlink
    if resolver.is_link() {
        return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Symlink access is restricted"));
    }

    let mut file = File::open(&resolved_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let shared_path = Arc::new(resolved_path);
    let sp_clone = Arc::clone(&shared_path);
    let handle = thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(10));
        if let Ok(mut f) = OpenOptions::new().write(true).open(&*sp_clone) {
            let _ = f.write_all(b"malicious update");
        }
    });
    handle.join().unwrap();
    Ok(content)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = if args.len() > 1 { &args[1] } else { "test.txt" };
    match process_file(file_path) {
        Ok(text) => println!("File content: {}", text),
        Err(e) => eprintln!("Error encountered: {}", e),
    }
}