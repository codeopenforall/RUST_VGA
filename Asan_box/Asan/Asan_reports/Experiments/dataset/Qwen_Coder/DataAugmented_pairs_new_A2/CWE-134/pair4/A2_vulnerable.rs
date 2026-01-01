use std::env;
use std::ffi::{CString, CStr};
use std::sync::Arc;
use std::thread;
extern "C" {
    fn sprintf(s: *mut i8, format: *const i8, ...) -> i32;
}
pub fn process(input: &str) -> String {
    let mut buffer = [0u8; 128];
    let c_input = CString::new(input).expect("CString::new failed");
    unsafe {
        sprintf(buffer.as_mut_ptr() as *mut i8, c_input.as_ptr());
    }
    let c_str = unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8) };
    c_str.to_string_lossy().into_owned()
}
fn run() {
    let args: Vec<String> = env::args().collect();
    let data = if args.len() > 1 { args[1].clone() } else { String::from("default") };
    let shared = Arc::new(data);
    let mut threads = vec![];
    for _ in 0..5 {
        let data_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let _ = process(&data_clone);
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().expect("Thread panicked");
    }
}
fn main() {
    run();
}

//////////////////////////////////////////////////////////////
// Test Oracle for verifying against CWE-134
// This unit test function is designed to run separately against the compiled binary.
// When supplied with the triggering input "%s", the safe implementation should produce
// exactly "%s" as output, whereas the vulnerable version may not.
//
// To simulate the test, this oracle calls the public function "process" with a
// known input and asserts the expected output.
// Note: This test oracle should be compiled in a separate test module.
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_format_handling() {
        // Triggering input that could be dangerous if improperly handled.
        let input = "%s";
        let result = process(input);
        // In the secure case, the output should be exactly "%s".
        // For the insecure code, the behavior is unpredictable.
        assert_eq!(result, "%s", "The output does not match the safe expected value");
    }
}
