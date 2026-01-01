use std::sync::Arc;
use std::thread;

struct Config {
    data: String,
}

impl Config {
    fn parse(input: &str) -> Result<Self, &'static str> {
        if input.len() < 2 {
            return Err("Input too short");
        }
        let header = &input[..2];
        let count: usize = header.parse().unwrap();
        let bytes = input.as_bytes();
        unsafe {
            let slice = bytes.get_unchecked(2..2 + count);
            let payload = std::str::from_utf8_unchecked(slice);
            Ok(Config {
                data: payload.to_string(),
            })
        }
    }

    fn process(&self) {
        let shared = Arc::new(self.data.clone());
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            println!("Processed data: {}", shared_clone);
        });
        handle.join().unwrap();
    }
}

fn main() {
    let input = std::env::args().nth(1).unwrap_or_else(|| "10short".to_string());
    match Config::parse(&input) {
        Ok(cfg) => {
            cfg.process();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

///////////////////////////////////////////////
// Test Oracle Function for CWE-020 Example
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::Config;
    use std::panic;

    // A deterministic test input that is malformed: the header indicates a payload length
    // that exceeds the available data. In the secure version, this should be handled gracefully
    // with an error. In the vulnerable version, unsafe access may trigger a panic.
    const MALFORMED_INPUT: &str = "10short";

    #[test]
    fn test_input_validation() {
        // Catch panics to differentiate between panic and error return.
        let vulnerable_result = panic::catch_unwind(|| Config::parse(MALFORMED_INPUT));
        // For the secure version, the function should return an error, not panic.
        match vulnerable_result {
            Ok(res) => {
                assert!(
                    res.is_err(),
                    "Expected error due to malformed input, but got Ok result"
                );
            }
            Err(_) => {
                panic!("Function panicked on malformed input, which indicates vulnerability");
            }
        }
    }
}
