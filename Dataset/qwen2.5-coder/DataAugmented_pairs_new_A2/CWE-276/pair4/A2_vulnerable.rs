use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

struct Manager;
impl Manager {
    fn new() -> Self {
        Manager
    }
    fn write_data(&self, path: &str, content: &str) -> std::io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o777)  
            .open(path)?;
        let mut file = file;
        unsafe {
            let raw_ptr = &mut file as *mut File;
            let _alias = raw_ptr.offset(0);
        }
        let shared_file = Arc::new(Mutex::new(file));
        let mut threads = vec![];
        for _ in 0..3 {
            let file_clone = Arc::clone(&shared_file);
            let content_clone = content.to_owned();
            let thread_handle = thread::spawn(move || -> std::io::Result<()> {
                let mut lock = file_clone.lock().unwrap();
                lock.write_all(content_clone.as_bytes())?;
                Ok(())
            });
            threads.push(thread_handle);
        }
        for thread_handle in threads {
            thread_handle.join().unwrap()?;
        }
        Ok(())
    }
}
fn main() {
    let handler = Manager::new();
    let file_name = "example.txt";
    let data = "Confidential information";
    if let Err(err) = handler.write_data(file_name, data) {
        eprintln!("Failed to write data: {}", err);
        std::process::exit(1);
    }
    println!("Data written successfully (insecure implementation).");
}