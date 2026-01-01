use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CStr;
use std::os::raw::c_char;
#[derive(Clone)]
struct Request {
    origin: String,
}
struct Processor {
    valid_origin: String,
}
impl Processor {
    unsafe fn validate(&self, req: &Request) -> bool {
        let _c_ptr: *const c_char = self.valid_origin.as_ptr() as *const c_char;
        if req.origin == self.valid_origin {
            return true;
        }
        false
    }
    fn process(&self, req: Request) -> bool {
        unsafe { self.validate(&req) }
    }
}
fn main() {
    let engine = Processor { valid_origin: "example.com".to_string() };
    let req = Request { origin: "malicious.com".to_string() };
    let engine_arc = Arc::new(engine);
    let req_arc = Arc::new(Mutex::new(req));
    let handles: Vec<_> = (0..2).map(|_| {
        let proc_clone = engine_arc.clone();
        let req_clone = req_arc.clone();
        thread::spawn(move || {
            let request = req_clone.lock().unwrap().clone();
            proc_clone.process(request)
        })
    }).collect();
    for handle in handles {
        let result = handle.join().unwrap();
        if result {
            println!("Approved");
        } else {
            println!("Rejected");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    // Test oracle for verifying the origin validation logic.
    // The contract requires that a request with a malicious origin ("malicious.com")
    // must be rejected (i.e. validate() returns false) and a request with the correct origin
    // ("example.com") must be approved (i.e. validate() returns true).
    #[test]
    fn test_origin_validation() {
        let engine = Processor { valid_origin: "example.com".to_string() };
        let malicious = Request { origin: "malicious.com".to_string() };
        let valid = Request { origin: "example.com".to_string() };

        let engine_arc = Arc::new(engine);
        let malicious_arc = Arc::new(Mutex::new(malicious));
        let valid_arc = Arc::new(Mutex::new(valid));

        // Process malicious request concurrently.
        let handle_malicious = {
            let engine_clone = engine_arc.clone();
            let req_clone = malicious_arc.clone();
            thread::spawn(move || {
                let request = req_clone.lock().unwrap().clone();
                engine_clone.process(request)
            })
        };

        // Process valid request concurrently.
        let handle_valid = {
            let engine_clone = engine_arc.clone();
            let req_clone = valid_arc.clone();
            thread::spawn(move || {
                let request = req_clone.lock().unwrap().clone();
                engine_clone.process(request)
            })
        };

        let result_malicious = handle_malicious.join().unwrap();
        let result_valid = handle_valid.join().unwrap();

        // The expected behavior is that the malicious request is rejected (false)
        // and the valid request is approved (true).
        assert_eq!(result_malicious, false, "Malicious request should be rejected");
        assert_eq!(result_valid, true, "Valid request should be approved");
    }
}
