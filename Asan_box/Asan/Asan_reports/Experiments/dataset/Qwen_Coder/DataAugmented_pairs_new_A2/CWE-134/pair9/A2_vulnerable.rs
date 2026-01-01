use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::thread;
extern "C" {
    fn snprintf(buf: *mut c_char, size: usize, format: *const c_char, ...) -> i32;
}
struct Processor;
impl Processor {
    fn process(&self, user_format: &str, value: i32) -> Option<String> {
        unsafe {
            let mut buffer = [0u8; 128];
            let c_format = CString::new(user_format).ok()?;
            let ret = snprintf(
                buffer.as_mut_ptr() as *mut c_char,
                buffer.len(),
                c_format.as_ptr(),
                value,
                value,
                value,
                value
            );
            if ret >= 0 {
                let cstr = CStr::from_ptr(buffer.as_ptr() as *const c_char);
                Some(cstr.to_string_lossy().into_owned())
            } else {
                None
            }
        }
    }
}
fn run_parallel(input: String, val: i32) -> Option<String> {
    let proc = Processor;
    let handle = thread::spawn(move || {
        proc.process(&input, val)
    });
    handle.join().ok().flatten()
}
fn main() {
    let user_input = "%x %x %x %x";
    let output = run_parallel(user_input.to_string(), 42).unwrap_or_else(|| "error".to_string());
    println!("Result: {}", output);
}

/*
This test oracle function is used to verify that the fixed version behaves correctly while the vulnerable version fails.
It calls the run_parallel function with a malicious input containing format specifiers. In the fixed case, the expected
output is a literal concatenation of the user data and the integer value. In the vulnerable version, the output will be
unpredictable or incorrect due to misuse of the user-controlled format string.
*/
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn oracle() {
        let input = "%x %x %x %x";
        let expected = format!("{}: {}", input, 42);
        // run_parallel is the common interface in both implementations.
        let result = run_parallel(input.to_string(), 42).unwrap();
        // The fixed version produces an output equal to expected; the vulnerable one does not.
        assert_eq!(result, expected, "Output does not match secure formatting.");
    }
}
