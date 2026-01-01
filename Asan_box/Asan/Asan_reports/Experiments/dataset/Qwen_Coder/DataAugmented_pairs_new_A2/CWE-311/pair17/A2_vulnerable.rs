use std::sync::{Arc, Mutex};
use std::thread;

struct Config {
    secret: String,
}

impl Config {
    fn new() -> Self {
        Config { secret: "SuperSecret".to_string() }
    }
}

unsafe fn transmit(raw_ptr: *const u8, len: usize) -> Vec<u8> {
    std::slice::from_raw_parts(raw_ptr, len).to_vec()
}

pub fn get_transmitted() -> Vec<u8> {
    let cfg = Arc::new(Mutex::new(Config::new()));
    let cfg_clone = Arc::clone(&cfg);
    let mut out = Vec::new();
    let handle = thread::spawn(move || {
        let guard = cfg_clone.lock().unwrap();
        let data = guard.secret.as_bytes();
        let transmitted = unsafe {
            transmit(data.as_ptr(), data.len())
        };
        transmitted
    });
    out = handle.join().unwrap();
    out
}

fn run() {
    let transmitted = get_transmitted();
    println!("Transmitted data: {}", String::from_utf8_lossy(&transmitted));
}

fn main() {
    run();
}

/* Test Oracle Function:
   This test verifies that the transmitted data is not the cleartext "SuperSecret".
   It should fail for the vulnerable version and pass for the fixed version.
*/

#[cfg(test)]
mod tests {
    // Import the get_transmitted function from the module under test.
    // Adjust the module name as necessary if the code is organized into separate crates or modules.
    use super::get_transmitted;

    #[test]
    fn test_sensitive_data_encryption() {
        let transmitted = get_transmitted();
        // If the transmitted bytes, when interpreted as UTF-8, equal "SuperSecret", then
        // encryption was not applied.
        assert_ne!(
            String::from_utf8_lossy(&transmitted),
            "SuperSecret",
            "Sensitive data is transmitted unencrypted!"
        );
    }
}
