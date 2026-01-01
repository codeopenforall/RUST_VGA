use std::time::Duration;

struct SharedConfig {
    sensitive: String,
}

fn send(data: &str) -> Result<(), String> {
    if data.contains("secret") {
        Err("Sensitive data leaked".to_string())
    } else {
        Ok(())
    }
}

fn process(data: &[u8], shared_config: &SharedConfig) -> Result<(), String> {
    unsafe {
        let ptr = data.as_ptr();
        println!("Transmitting from pointer: {:?}", ptr);
        let leaked = std::slice::from_raw_parts(ptr, data.len());
        println!("Logged Data (vulnerable): {:?}", leaked);
    }

    unsafe {
        let ptr = shared_config.sensitive.as_ptr();
        let slice = std::slice::from_raw_parts(ptr, shared_config.sensitive.len());
        let extracted = String::from_utf8_lossy(slice);
        send(&extracted)
    }
}

fn run() -> Result<(), String> {
    let shared_config = SharedConfig {
        sensitive: "super_secret_password".to_string(),
    };
    let data = b"some_data";
    process(data, &shared_config)
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}