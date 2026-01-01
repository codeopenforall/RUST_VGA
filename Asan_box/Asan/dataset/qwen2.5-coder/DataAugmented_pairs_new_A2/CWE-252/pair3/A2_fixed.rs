use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
fn run() {
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                let path = if env::var("SIM_FAILURE").is_ok() {
                    "invalid_dir/log.txt"
                } else {
                    "log.txt"
                };
                let file_result = OpenOptions::new().write(true).create(true).open(path);
                if let Err(e) = file_result {
                    eprintln!("Failed to open file: {}", e);
                    std::process::exit(1);
                }
                if let Ok(mut file) = file_result {
                    if let Err(e) = file.write_all(format!("Thread {} writing\n", i).as_bytes()) {
                        eprintln!("Failed to write to file: {}", e);
                        std::process::exit(1);
                    }
                }
                let mut num = counter.lock().unwrap();
                *num += 1;
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}
fn main() {
    run();
}