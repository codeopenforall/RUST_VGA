use std::sync::{Arc, Mutex};
use std::thread;
use std::os::raw::c_char;
use std::ffi::CString;
extern "C" {
    fn printf(format: *const c_char, ...) -> i32;
}
struct Data {
    info: String,
}
trait Displayable {
    fn show(&self);
}
impl Displayable for Data {
    fn show(&self) {
        let raw_input = self.info.clone();
        let safe_format = CString::new(raw_input).expect("CString creation failed");
        unsafe {
            printf(safe_format.as_ptr());
        }
    }
}
fn handle_message(user_str: String) -> Result<(), String> {
    let data = Data { info: user_str };
    let shared = Arc::new(Mutex::new(data));
    let threads: Vec<_> = (0..3).map(|_| {
        let cloned = shared.clone();
        thread::spawn(move || {
            let locked = cloned.lock().expect("Locking failed");
            locked.show();
        })
    }).collect();
    for t in threads {
        t.join().map_err(|_| "Thread join failed".to_string())?;
    }
    Ok(())
}
fn main() {
    let user_input = "%s%s%s";
    handle_message(user_input.to_string()).expect("Processing failed");
}

//////////////////////////////////////
// Test Oracle for Format String Issue
//////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This test verifies that the function properly handles a malicious format string.
    // In the vulnerable implementation, passing a malicious format string will lead to undefined behavior,
    // e.g., a panic or a crash. In the fixed implementation, the formatting is safely decoupled.
    #[test]
    fn test_format_safety() {
        let malicious = "%s%s%s";
        
        // Use catch_unwind to observe any panic (indicative of a vulnerability exploit).
        let result = panic::catch_unwind(|| {
            handle_message(malicious.to_string()).unwrap();
        });
        // The test passes only if no panic occurs.
        assert!(result.is_ok(), "Unexpected panic due to format string vulnerability");
    }
}
