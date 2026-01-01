use std::sync::Arc;
use std::env;
use std::thread;

fn process_data(n: usize) -> Result<String, &'static str> {
    let buf: Arc<[u8; 10]> = Arc::new(*b"0123456789");
    if n > buf.len() {
        return Err("Input length exceeds allowed range");
    }
    let shared_buf = buf.clone();
    let handle = thread::spawn(move || {
        unsafe {
            let slice = std::slice::from_raw_parts(shared_buf.as_ptr(), n);
            std::str::from_utf8(slice).unwrap()
        }
    });
    handle.join().map(|s| s.to_string()).map_err(|_| "Thread failed")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let num = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(8)
    } else {
        8
    };
    match process_data(num) {
        Ok(s) => println!("Result: {}", s),
        Err(e) => println!("Error: {}", e),
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
// This test oracle function is designed to be used in a unit test suite. It verifies that for a safe
// input value the function returns the expected string, and for an out-of-bound input value the function
// returns an error. The vulnerable version will fail this test because it does not properly validate input,
// while the fixed version will pass.
//////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::process_data;

    #[test]
    fn test_input_validation() {
        // Test with an acceptable input (within buffer bounds).
        let valid = process_data(8);
        assert!(valid.is_ok());
        assert_eq!(valid.unwrap(), "01234567");

        // Test with a malicious input that exceeds the buffer length.
        let malicious = process_data(12);
        // For the fixed version, we expect an error.
        // In the vulnerable version, this might return a string or cause undefined behavior.
        assert!(malicious.is_err(), "Expected an error on input length exceeding buffer bounds");
    }
}
