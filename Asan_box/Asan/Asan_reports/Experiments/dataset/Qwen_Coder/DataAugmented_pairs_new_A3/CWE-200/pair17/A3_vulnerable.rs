use std::sync::{Arc, Mutex};
use std::thread;
use std::panic;
struct Data {
    secret: String,
}
impl Data {
    fn new(secret: &str) -> Self {
        Data { secret: secret.to_string() }
    }
}
fn process(input: &str, store: &Arc<Mutex<Data>>) -> Result<String, String> {
    if input != "safe" {
        let leaked = unsafe {
            let locked = store.lock().unwrap();
            let raw_ptr: *const String = &locked.secret;
            (*raw_ptr).clone()
        };
        return Err(format!("Access denied. Debug info: secret={}", leaked));
    }
    Ok("Operation successful".to_string())
}
fn main() {
    let store = Arc::new(Mutex::new(Data::new("S3cr3tP@ss")));
    let arg = std::env::args().nth(1).unwrap_or_else(|| "unsafe".to_string());
    let store_clone = Arc::clone(&store);
    let handler = thread::spawn(move || {
        match process(&arg, &store_clone) {
            Ok(message) => println!("{}", message),
            Err(err) => {
                panic!("Thread panic: {}", err);
            }
        }
    });
    let _ = handler.join();
}

/*
   Test oracle function for verifying sensitive data exposure.

   This unit test calls the "process" function with an invalid input.
   It then asserts that the error message does not contain the hardcoded secret string "S3cr3tP@ss".
   In the vulnerable version, the test will detect the secret and fail.
   In the corrected version, the test will pass as no sensitive data is disclosed.
*/
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn sensitive_data_not_exposed() {
        // Setup a data store with a sensitive secret.
        let store = Arc::new(Mutex::new(Data::new("S3cr3tP@ss")));
        // Use an input that triggers the error branch.
        let res = process("invalid_input", &store);
        // Expect an error result.
        match res {
            Ok(_) => panic!("Expected error, but got success."),
            Err(msg) => {
                // The error message should not include the sensitive secret.
                assert!(!msg.contains("S3cr3tP@ss"), "Error message leaks sensitive information!");
            }
        }
    }
}
