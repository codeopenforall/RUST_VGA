use std::sync::Arc;
use std::thread;

struct Processor {}

impl Processor {
    fn new() -> Processor {
        Processor {}
    }

    fn process(&self, user_input: &str) -> String {
        // Ensure the input is treated as a literal string
        user_input.to_string()
    }
}

fn main() {
    let proc = Arc::new(Processor::new());
    let proc_clone = Arc::clone(&proc);
    let handle = thread::spawn(move || {
        let res = proc_clone.process("%x %x %x");
        println!("{}", res);
    });
    handle.join().unwrap();
}

////////////////////////////
// Test Oracle Function
////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    // This test oracle is intended to be used with both variants.
    // The safe version (fixed) produces an output exactly equal to the input.
    // The vulnerable version misinterprets the input format specifiers, so the output will differ.
    #[test]
    fn test_format_integrity() {
        let input = "Test literal: %s %x";
        let expected = "Test literal: %s %x";
        let proc = Processor::new();
        let output = proc.process(input);
        // For the safe version, the assertion should pass.
        // For the vulnerable version, the formatting function interprets the specifiers
        // and the output will not match the expected literal string.
        assert_eq!(output, expected, "The output should exactly match the literal input.");
    }
}
