use std::env;
use std::str;
use std::slice;
use std::thread;

struct Processor;

impl Processor {
    fn process(&self, input: &str) -> Result<String, String> {
        let mut parts = input.splitn(2, ' ');
        let len_str = parts.next().ok_or("Missing length token")?;
        let data = parts.next().ok_or("Missing payload token")?;
        let expected_len: usize = len_str.parse().map_err(|_| "Invalid length token")?;
        
        if expected_len > data.len() {
            return Err("Length exceeds payload size".to_string());
        }

        unsafe {
            let ptr = data.as_ptr();
            let unslice = slice::from_raw_parts(ptr, expected_len);
            let result = str::from_utf8_unchecked(unslice);
            Ok(result.to_owned())
        }
    }
}

fn main() {
    let processor = Processor;
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <expected_length> <payload>", args[0]);
        return;
    }
    let input = format!("{} {}", args[1], args[2]);
    let handle = thread::spawn(move || {
        match processor.process(&input) {
            Ok(out)  => println!("Output: {}", out),
            Err(err) => println!("Error: {}", err),
        }
    });
    handle.join().unwrap();
}

////////////////////////////////////////////////////////////
// Test Oracle Function for Vulnerability Validation
////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Processor;

    // The test passes for the corrected version because it will return an error when:
    //  - The specified length exceeds the actual payload length.
    // For the vulnerable version, the unsafe block might produce unpredictable results,
    // causing the test to fail consistently.
    #[test]
    fn test_input_length_validation() {
        let processor = Processor;
        // Input where the expected length (10) is greater than the payload length ("hello" is 5).
        let input = "10 hello";
        let result = processor.process(input);
        // For the fixed version, we expect an explicit error message.
        assert!(result.is_err(), "Expected an error due to length exceeding payload size");
        assert_eq!(result.unwrap_err(), "Length exceeds payload size");
    }
}
////////////////////////////////////////////////////////////
// End of Test Oracle Function
////////////////////////////////////////////////////////////
