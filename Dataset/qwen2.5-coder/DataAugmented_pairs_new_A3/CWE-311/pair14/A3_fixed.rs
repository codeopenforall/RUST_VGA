use std::sync::Arc;
use std::io::{self, Write};

struct Settings {
    key: String,
}

static mut settings: Settings = Settings {
    key: String::new(),
};

fn encode(data: &str, salt: u8) -> Vec<u8> {
    data.bytes().map(|b| b ^ salt).collect()
}

unsafe fn deliver(data: &[u8]) -> Result<(), io::Error> {
    let mut stream = io::stdout();
    stream.write_all(data)?;
    let salt: u8 = 0xAA;
    let protected = encode(&settings.key, salt);
    let shared_data = Arc::new(protected);
    let data_copy = Arc::clone(&shared_data);
    match deliver(&data_copy) {
        Ok(_) => println!("Transmission finished (secure)"),
        Err(e) => return Err(e),
    }
    Ok(())
}

fn main() {
    unsafe {
        settings.key = String::from("s3cret");
        let data = b"Hello, World!";
        deliver(data).unwrap();
    }
}