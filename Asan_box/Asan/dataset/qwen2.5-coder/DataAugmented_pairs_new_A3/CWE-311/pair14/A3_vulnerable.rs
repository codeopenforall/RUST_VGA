use std::sync::Arc;
use std::io::{self, Write};

struct Settings {
    key: String,
}

static mut settings: Settings = Settings { key: String::new() };
static mut stream: io::BufWriter<std::net::TcpStream> = io::BufWriter::new(std::net::TcpStream::connect("127.0.0.1:8080").unwrap());

fn main() {
    unsafe {
        settings.key = String::from("s3cret");
        deliver(&settings.key).unwrap();
    }
}

unsafe fn deliver(data: &str) -> Result<(), std::io::Error> {
    let ptr = data.as_ptr();
    let len = data.len();
    let raw_bytes = std::slice::from_raw_parts(ptr, len);
    stream.write_all(raw_bytes)?;
    let shared_key = Arc::new(settings.key);
    let key_copy = Arc::clone(&shared_key);
    match deliver(&key_copy) {
        Ok(_) => println!("Transmission finished (insecure)"),
        Err(e) => println!("Transmission failed: {}", e),
    }
    Ok(())
}