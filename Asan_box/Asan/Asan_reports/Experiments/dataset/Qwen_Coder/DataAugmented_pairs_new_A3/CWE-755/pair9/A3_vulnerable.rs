use std::sync::{Arc, Mutex};
use std::thread;
use std::fs::File;
use std::io::{Read, Error as IOError};
use std::error::Error;
#[derive(Debug)]
struct AppSettings {
    secure_mode: bool,
    value: i32,
}
impl Default for AppSettings {
    fn default() -> Self {
        AppSettings { secure_mode: false, value: 0 }
    }
}
fn load_settings() -> Result<AppSettings, Box<dyn Error>> {
    let mut file = File::open("config.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    unsafe {
        let raw_ptr = content.as_ptr();
        if raw_ptr.is_null() {
            return Err("Null pointer encountered".into());
        }
    }
    let parsed_value = content.trim().parse::<i32>().unwrap_or_default(); // CWE-755: Swallowing error
    let secure = parsed_value == 1;
    Ok(AppSettings { secure_mode: secure, value: parsed_value })
}
fn main() -> Result<(), Box<dyn Error>> {
    let settings = Arc::new(Mutex::new(AppSettings::default()));
    let settings_clone = Arc::clone(&settings);
    let handle = thread::spawn(move || {
        let new_settings = load_settings().expect("Failed to load config settings");
        let mut s = settings_clone.lock().unwrap();
        *s = new_settings;
    });
    handle.join().expect("Thread panicked");
    let s = settings.lock().unwrap();
    if s.secure_mode {
        println!("Running in secure mode with value {}", s.value);
    } else {
        println!("Running in insecure mode with value {}", s.value);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, remove_file};
    use std::io::Write;
    
    // Test oracle that expects a configuration parsing error.
    // For the vulnerable version, this test will fail,
    // since malformed input will be silently replaced by defaults.
    // For the corrected version, an error is properly returned.
    #[test]
    fn oracle() {
        // Create a configuration file with malformed content.
        let filename = "config.txt";
        let mut file = File::create(filename).expect("Could not create config file");
        // The content "notanumber" cannot be parsed as i32.
        file.write_all(b"notanumber").expect("Failed to write to config file");
        
        let result = load_settings();
        // The test expects a parse error. In the vulnerable version,
        // load_settings would return Ok(AppSettings { value: 0, secure_mode: false }).
        assert!(result.is_err(), "Expected error for malformed configuration data");
        
        remove_file(filename).expect("Could not remove config file");
    }
}
