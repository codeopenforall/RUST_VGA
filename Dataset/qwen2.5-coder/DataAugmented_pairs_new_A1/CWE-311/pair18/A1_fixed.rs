use std::fs::File;
use std::io::Write;
use std::ptr;

struct Settings {
    secret: String,
}

impl Settings {
    unsafe fn expose(&self) -> &[u8] {
        let ptr: *const u8 = self.secret.as_ptr();
        let len = self.secret.len();
        std::slice::from_raw_parts(ptr, len)
    }
}

fn store_data(data: &[u8]) -> std::io::Result<()> {
    let mut file = File::create("config.txt")?;
    file.write_all(data)?;
    Ok(())
}

fn run_main() {
    let settings = Settings { secret: "SuperSecret!".to_string() };
    let data = unsafe { settings.expose() };
    let encrypted_data: Vec<u8> = data.iter().map(|&b| b ^ 0xAA).collect();
    if let Err(e) = store_data(&encrypted_data) {
        eprintln!("Error storing configuration: {}", e);
    }
}

fn main() {
    run_main();
}