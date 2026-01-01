use std::fs::{self, File};
use std::io::{self, Read};
use std::env;
use std::os::unix::fs::MetadataExt;
use std::thread;
use std::time::Duration;

fn process_file(path: &str) -> io::Result<String> {
    let info = fs::metadata(path)?;
    if !info.is_file() {
        return Err(io::Error::new(io::ErrorKind::Other, "Not a file"));
    }
    // Check metadata again after a short sleep to detect TOCTOU vulnerability
    thread::sleep(Duration::from_millis(50));
    let info_after_sleep = fs::metadata(path)?;
    if info.dev() != info_after_sleep.dev() || info.ino() != info_after_sleep.ino() {
        return Err(io::Error::new(io::ErrorKind::Other, "TOCTOU detected"));
    }
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    match process_file(&args[1]) {
        Ok(content) => println!("File content:\n{}", content),
        Err(e) => {
            eprintln!("Error processing file: {:?}", e);
            std::process::exit(1);
        }
    }
}