use std::ptr;
use std::thread;
fn process() -> String {
    let original = String::from("This is a test message that exceeds the typical struct size.");
    let output = thread::spawn(move || {
        unsafe {
            let copy_len = original.len() - 5; // Introduce a bug by reducing the copy length
            let mut buffer: Vec<u8> = Vec::with_capacity(copy_len);
            buffer.set_len(copy_len);
            ptr::copy_nonoverlapping(original.as_ptr(), buffer.as_mut_ptr(), copy_len);
            String::from_utf8_lossy(&buffer).into_owned()
        }
    }).join().unwrap();
    output
}
fn main() {
    let result = process();
    println!("{}", result);
}

///////////////////////////////////////
// Test Oracle Function
///////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::process::{Command, Stdio};
    use std::io::Read;

    // Expected complete string
    const EXPECTED: &str = "This is a test message that exceeds the typical struct size.";

    // Test the process() function directly.
    #[test]
    fn test_process_output() {
        let output = process();
        // The vulnerable version would produce a truncated string, so this test will fail for it,
        // whereas the fixed version produces the complete expected string.
        assert_eq!(output, EXPECTED, "The output string does not match the expected full string");
    }
}
///////////////////////////////////////
// End of Test Oracle Function
///////////////////////////////////////
