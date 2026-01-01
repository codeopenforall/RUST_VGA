use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::sync::{Arc, Mutex};
use std::thread;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

struct App {
    data: Arc<Mutex<String>>,
}

impl App {
    fn new() -> Self {
        App {
            data: Arc::new(Mutex::new(String::new())),
        }
    }

    fn create_resource<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o600) // Change mode to 0o600 for secure permissions
            .open(&path)?;
        drop(file);
        Ok(())
    }

    fn process(&self, path: &str) {
        let pathname = path.to_owned();
        let resource = Arc::clone(&self.data);
        let handler = thread::spawn(move || {
            let inst = App { data: resource };
            if let Err(e) = inst.create_resource(&pathname) {
                eprintln!("Resource creation failed: {}", e);
            }
        });
        handler.join().unwrap();
    }
}

fn main() {
    let instance = App::new();
    let test_path = "/tmp/insecure_resource.txt";
    instance.process(test_path);
    println!("Operation complete.");
}