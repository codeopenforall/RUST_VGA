use std::env;
use std::ffi::{CString, CStr};
use std::sync::{Arc, Mutex};
use std::thread;
extern "C" {
    fn snprintf(s: *mut i8, n: usize, format: *const i8, ...) -> i32;
}
fn process(user: &str) -> String {
    let mut buffer = [0u8; 256];
    let c_format = CString::new(user).expect("Conversion to CString failed");
    unsafe {
        snprintf(
            buffer.as_mut_ptr() as *mut i8,
            buffer.len(),
            c_format.as_ptr(),
        );
    }
    let c_str = unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8) };
    c_str.to_string_lossy().into_owned()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "normal_input" };
    let shared_result = Arc::new(Mutex::new(String::new()));
    let mut handles = vec![];
    for _ in 0..4 {
        let input_clone = input.to_string();
        let shared_clone = Arc::clone(&shared_result);
        let handle = thread::spawn(move || {
            let res = process(&input_clone);
            let mut data = shared_clone.lock().unwrap();
            *data = res;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    let final_result = shared_result.lock().unwrap();
    println!("Result: {}", *final_result);
}

/////////////////////////////////////////////////////////////
// Test Oracle Function (separate from the main programs)
// This test should fail for the vulnerable version and pass for the corrected version.
/////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // Import the process function from the module under test.
    // In the vulnerable build, this test is expected to fail due to unsafe formatting.
    // In the corrected build, the test will pass.
    use super::process;

    #[test]
    fn test_format_safety() {
        // Malicious input containing format specifiers.
        let input = "malicious %x %x %x";
        // In the fixed version, the output should exactly echo the malicious input.
        let expected = "malicious %x %x %x";
        let output = process(input);
        assert_eq!(output, expected, "Output does not match expected safe formatting behavior");
    }
}
