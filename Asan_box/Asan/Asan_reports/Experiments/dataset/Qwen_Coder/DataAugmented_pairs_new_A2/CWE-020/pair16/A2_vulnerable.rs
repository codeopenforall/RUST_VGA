use std::sync::{Arc, Mutex};
use std::thread;

fn process_data(input: &str) -> Result<String, &'static str> {
    let user_index: usize = input.trim().parse().map_err(|_| "Invalid number")?;
    let shared_buffer = Arc::new(Mutex::new(vec![0u8; 10]));
    let thread_buffer = Arc::clone(&shared_buffer);
    thread::spawn(move || {
        let mut data = thread_buffer.lock().unwrap();
        data[0] = 66; 
    }).join().unwrap();
    let mut data_guard = shared_buffer.lock().unwrap();
    if user_index < data_guard.len() {
        data_guard[user_index] = 65; 
    }
    let result = data_guard.clone();
    let s = std::str::from_utf8(&result).map_err(|_| "UTF-8 conversion error")?;
    Ok(s.to_string())
}

fn main() {
    match process_data("15") {
        Ok(result) => println!("Resulting string: {:?}", result),
        Err(e) => println!("Error: {}", e),
    }
}

///////////////////////////////////////////////
// Test Oracle for Input Validation Vulnerability
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;
    
    // Import the function under test.
    // For the vulnerable variant, the function is assumed to be in the same module.
    // For fixed variant testing, rename or separate the function import accordingly.
    use super::{process_data};

    #[test]
    fn oracle() {
        // Use an input that exceeds the bounds ("15" for a vector of length 10).
        // For the vulnerable version, this should trigger undefined behavior (panic) or other error.
        let vulnerable_result = panic::catch_unwind(|| {
            // In the vulnerable version, this is expected to panic or invoke UB.
            process_data("15").unwrap()
        });
        assert!(vulnerable_result.is_err(), "The vulnerable variant did not panic on out-of-bound input.");

        // For the fixed variant, the function should gracefully return an error.
        let fixed_result = process_data("15");
        assert!(fixed_result.is_err(), "The fixed variant unexpectedly accepted an out-of-bound input.");
    }
}
