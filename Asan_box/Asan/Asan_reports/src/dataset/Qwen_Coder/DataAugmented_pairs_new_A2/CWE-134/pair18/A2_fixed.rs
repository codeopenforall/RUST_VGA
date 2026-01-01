use std::env;
use std::ffi::{CString, CStr};
use std::ptr;
use std::sync::Arc;
use std::thread;
extern "C" {
    fn snprintf(buf: *mut i8, size: usize, format: *const i8, ...) -> i32;
}
fn process(input: &str) -> String {
    let fixed_format = CString::new("%s").expect("CString conversion failed for fixed format");
    let c_input = CString::new(input).expect("CString conversion failed for user input");
    let mut buffer = [0i8; 256];
    unsafe {
        let _ = snprintf(buffer.as_mut_ptr(), buffer.len(), fixed_format.as_ptr(), c_input.as_ptr());
        CStr::from_ptr(buffer.as_ptr()).to_string_lossy().into_owned()
    }
}
fn run_app() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input>", args[0]);
        return;
    }
    let user_input = args[1].clone();
    let shared = Arc::new(user_input);
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || process(&shared_clone));
    let result = handle.join().unwrap();
    println!("{}", result);
}
fn main() {
    run_app();
}

/*
Test oracle function: This unit test calls the processing function with a malicious input string containing format specifiers.
For the vulnerable version, the unsafe use of the input causes undefined behavior (e.g., incorrect or unpredictable output),
and the test assertion (that the output exactly matches the raw input) fails.
For the fixed version, the user input is safely printed via the "%s" format string,
so the output exactly matches the input, and the test passes.

Note: This oracle function is independent from the main applications above.
*/

#[cfg(test)]
mod tests {
    use super::*;
    
    // Dummy process function signature re-imported from the code (vulnerable or fixed version).
    // For testing, ensure to call the same "process" function from the compiled version.
    #[test]
    fn test_format_handling() {
        // Malicious input that includes format specifiers.
        let input = "User input: %x %x";
        let output = process(input);
        // In the fixed version, output should simply be the input.
        // In the vulnerable version, embedded specifiers may lead to altered output.
        assert_eq!(output, input, "The processed output does not match the expected safe output");
    }
}
