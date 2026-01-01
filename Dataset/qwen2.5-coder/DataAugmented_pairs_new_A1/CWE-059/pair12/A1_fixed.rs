use std::sync::Arc;
use std::thread;

struct Manager {
    base: String,
}

impl Manager {
    fn new(base: &str) -> Self {
        Manager { base: base.to_string() }
    }

    fn open(&self, input: &str) -> Result<String, &'static str> {
        let full_path = format!("{}{}", self.base, input);
        if full_path.contains("..") {
            Err("Access denied")
        } else {
            Ok("regular_data".to_owned())
        }
    }
}

fn main() {
    let manager = Arc::new(Manager::new("/safe_dir/"));
    let mgr_clone = Arc::clone(&manager);
    let handle = thread::spawn(move || {
        let result = mgr_clone.open("../etc/passwd");
        println!("Thread result: {:?}", result);
    });
    handle.join().unwrap();
    match manager.open("file.txt") {
        Ok(content) => println!("Main thread content: {}", content),
        Err(e) => println!("Error: {}", e),
    }
}