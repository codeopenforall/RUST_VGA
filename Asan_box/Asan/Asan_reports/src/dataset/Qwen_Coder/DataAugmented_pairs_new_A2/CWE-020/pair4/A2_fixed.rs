use std::env;
static DATA: &[u8] = b"Example static data used for unsafe string extraction vulnerability demo";
struct Processor;
impl Processor {
    fn process(&self, offset: usize, len: usize) -> Result<String, String> {
        if offset + len > DATA.len() {
            return Err("Invalid input: offset and length exceed data bounds".to_string());
        }
        unsafe {
            let ptr = DATA.as_ptr().add(offset);
            let slice = std::slice::from_raw_parts(ptr, len);
            let s = std::str::from_utf8_unchecked(slice);
            Ok(s.to_string())
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} offset:len", args[0]);
        return;
    }
    let parts: Vec<&str> = args[1].split(':').collect();
    if parts.len() != 2 {
        println!("Invalid input format, expected offset:len");
        return;
    }
    let offset = parts[0].parse::<usize>().unwrap_or(0);
    let len = parts[1].parse::<usize>().unwrap_or(0);
    let proc_inst = Processor;
    match proc_inst.process(offset, len) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_extraction_valid_and_invalid() {
        let proc_inst = Processor;

        // Valid input: extract "Example" from the beginning.
        match proc_inst.process(0, 7) {
            Ok(result) => assert_eq!(result, "Example", "Valid extraction result mismatch"),
            Err(e) => panic!("Unexpected error on valid input: {}", e),
        }

        // Invalid input: intentionally request an out-of-bound slice.
        // For the fixed version, this must return an Err with a specific message.
        // The vulnerable version may panic unexpectedly or produce undefined data.
        let invalid_output = panic::catch_unwind(|| proc_inst.process(65, 10));
        match invalid_output {
            Ok(res) => {
                match res {
                    Ok(s) => panic!("Test failed: extraction with invalid input succeeded with '{}'", s),
                    Err(msg) => assert_eq!(msg, "Invalid input: offset and length exceed data bounds", "Unexpected error message"),
                }
            },
            Err(_) => panic!("Test failed: extraction with invalid input panicked in the vulnerable version"),
        }
    }
}
