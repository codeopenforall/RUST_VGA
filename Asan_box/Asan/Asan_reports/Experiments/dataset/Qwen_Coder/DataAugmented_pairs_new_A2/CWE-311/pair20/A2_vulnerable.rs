use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

struct Config {
    secret: String,
}

impl Config {
    fn new(secret: String) -> Self {
        Config { secret }
    }
}

fn process_config(conf: Arc<Mutex<Config>>) {
    let conf_copy = Arc::clone(&conf);
    let handler = thread::spawn(move || {
        let data = conf_copy.lock().unwrap();
        unsafe {
            let ptr = data.secret.as_ptr();
            let len = data.secret.len();
            let raw_slice = std::slice::from_raw_parts(ptr, len);
            let mut file = File::create("output.txt")
                .expect("Unable to create output file");
            file.write_all(raw_slice)
                .expect("Failed to write sensitive data to file");
        }
    });
    handler.join().unwrap();
}

fn main() {
    let config = Arc::new(Mutex::new(Config::new("SensitiveSecret".to_string())));
    process_config(config);
}

#[cfg(test)]
mod tests {
    use std::fs;
    // To ensure the test is reproducible, remove any pre-existing output file.
    fn cleanup() {
        let _ = fs::remove_file("output.txt");
    }

    // The common oracle test function.
    // This test calls the main function from the binary under test and checks
    // whether the sensitive data is stored in encrypted form.
    // For the vulnerable version, the file will contain the plaintext secret,
    // causing the assertion to fail. The fixed version will pass as the data is encrypted.
    #[test]
    fn test_sensitive_storage() {
        cleanup();
        // Call the main function of the binary.
        // In integration scenarios, this could be executed in a subprocess.
        // Here we assume a direct call to main() is possible.
        crate::main();
        let content = fs::read("output.txt").expect("Failed to read output file");

        // The expected sensitive string in plaintext.
        let sensitive = b"SensitiveSecret";

        // If data is unencrypted, it will match the plaintext.
        // To pass, the stored content must not equal the raw sensitive data.
        assert_ne!(
            content, sensitive,
            "Test failed: Sensitive data is stored in plaintext"
        );
    }
}
