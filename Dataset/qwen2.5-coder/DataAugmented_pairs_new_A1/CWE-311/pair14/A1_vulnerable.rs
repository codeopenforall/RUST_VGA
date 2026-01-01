use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::thread;

struct Config {
    secret: String,
}

impl Config {
    fn initialize(secret: String) -> Self {
        Self { secret }
    }
}

fn fetch_config() -> Config {
    let mut file = File::open("config.txt").expect("Cannot open config.txt");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed reading config.txt");
    Config::initialize(content)
}

unsafe fn transmit(data: &str) -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let ptr = data.as_ptr();
    let len = data.len();
    let raw_bytes = std::slice::from_raw_parts(ptr, len);
    stream.write_all(raw_bytes)?;
    Ok(())
}

fn main() {
    let config = fetch_config();
    let shared_secret = Arc::new(config.secret);
    let secret_copy = Arc::clone(&shared_secret);
    let handle = thread::spawn(move || {
        unsafe {
            match transmit(&secret_copy) {
                Ok(_) => println!("Transmission finished (insecure)"),
                Err(e) => println!("Transmission error: {:?}", e),
            }
        }
    });
    handle.join().unwrap();
}