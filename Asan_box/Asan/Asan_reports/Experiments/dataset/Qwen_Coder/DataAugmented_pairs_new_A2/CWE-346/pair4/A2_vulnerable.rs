use std::collections::HashMap;
use std::sync::{Arc};
use std::thread;
use std::ffi::CStr;
pub struct Request {
    pub headers: HashMap<String, String>,
}
impl Request {
    pub fn check_origin(&self) -> bool {
        let trusted = "https://example.com";
        if let Some(origin) = self.headers.get("Origin") {
            unsafe {
                let ptr = origin.as_ptr() as *const i8;
                let cstr_origin = CStr::from_ptr(ptr);
                match cstr_origin.to_str() {
                    Ok(s) => s.starts_with(trusted),
                    Err(_) => false,
                }
            }
        } else {
            false
        }
    }
}
fn process_request(req: Request) {
    let shared_req = Arc::new(req);
    let mut handles = Vec::new();
    for _ in 0..5 {
        let thread_req = Arc::clone(&shared_req);
        let handle = thread::spawn(move || {
            if thread_req.check_origin() {
                println!("Origin accepted. Proceeding with request processing.");
            } else {
                println!("Origin rejected. Aborting request.");
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
fn main() {
    let mut hdrs = HashMap::new();
    hdrs.insert("Origin".to_string(), "https://example.com\0.malicious".to_string());
    let req = Request { headers: hdrs };
    process_request(req);
}

//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // This test supplies a header with an embedded null byte.
    // For the vulnerable implementation, the unsafe CStr conversion would truncate the string
    // and mistakenly allow the request (test failure). The fixed version correctly rejects it.
    #[test]
    fn test_origin_validation() {
        let mut hdrs = HashMap::new();
        // Crafted input: The embedded null byte causes the CStr conversion
        // to only consider "https://example.com" in the vulnerable case.
        hdrs.insert("Origin".to_string(), "https://example.com\0.malicious".to_string());
        let req = Request { headers: hdrs };

        // The check should return false for a properly validated header.
        let result = req.check_origin();
        assert!(!result, "The request should be rejected due to invalid origin header.");
    }
}
