use std::thread;
use std::sync::Arc;
use std::error::Error;

#[derive(Debug)]
struct AppConfig {
    secret: String,
}

struct Server {
    config: Arc<AppConfig>,
}

impl Server {
    fn new(secret: &str) -> Server {
        Server { 
            config: Arc::new(AppConfig { secret: secret.to_string() }) 
        }
    }

    fn process_request(&self, request: &str) -> Result<String, Box<dyn Error>> {
        if request.is_empty() {
            unsafe {
                let ptr = self.config.secret.as_ptr();
                let _dummy = std::slice::from_raw_parts(ptr, 4);
            }
            Err(format!("Processing error: invalid configuration: {:?}", self.config).into())
        } else {
            Ok(format!("Processed: {}", request))
        }
    }
}

fn main() {
    let server = Server::new("SuperSecretKey123");
    let requests = vec!["valid_request", ""];
    let mut handles = vec![];
    for req in requests {
        let shared_cfg = server.config.clone();
        let r = req.to_string();
        let handle = thread::spawn(move || {
            if r.is_empty() {
                let config_snapshot = AppConfig { 
                    secret: String::from_utf8_lossy(shared_cfg.secret.as_bytes()).to_string() 
                };
                unsafe {
                    let secret_ptr = config_snapshot.secret.as_ptr();
                    let _unsafe_slice = std::slice::from_raw_parts(secret_ptr, 5);
                }
                panic!("Error: condition failed with configuration: {:?}", config_snapshot);
            } else {
                println!("Request succeeded: {}", r);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
}

///////////////////////////////////////////////////////
// Test Oracle Function using Rust's built-in test framework.
// This test is designed to catch sensitive information leaks.
// For the vulnerable version, the panic/error messages include "SuperSecretKey123"
// and the test will fail, whereas the fixed version uses generic messages and will pass.
///////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;
    use std::sync::Arc;
    use std::error::Error;

    // A minimal Server and AppConfig interface expected by the test.
    // For testing purposes, ensure these symbols are accessible from the respective binaries.
    // The test will call process_request with an empty input to force an error.

    // The oracle test function.
    #[test]
    fn test_sensitive_data_exposure() {
        // Instantiate a Server object with known secret.
        let secret = "SuperSecretKey123";
        // Instead of calling main (which spawns threads and panics), we instantiate the object directly.
        let server = crate::Server::new(secret);

        // Call process_request to generate an error.
        let err_result = server.process_request("");
        match err_result {
            Ok(_) => panic!("Expected error was not returned."),
            Err(e) => {
                let err_str = e.to_string();
                // Test contract:
                // For the vulnerable code, the error message would include the secret.
                // For the fixed code, it will not.
                if err_str.contains(secret) {
                    panic!("Sensitive data exposed in error message.");
                }
                // Otherwise, the fixed code passes.
                assert!(true);
            }
        }
    }
}
