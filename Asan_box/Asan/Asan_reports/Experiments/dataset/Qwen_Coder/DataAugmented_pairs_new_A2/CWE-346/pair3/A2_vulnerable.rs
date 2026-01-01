use std::ffi::CStr;
use std::thread;
use std::sync::Arc;
static TRUSTED: &[u8] = b"api.trusted.com\0";
struct Request {
    header: String,
    body: String,
}
fn process_request(req: Request) -> Result<(), &'static str> {
    let req_arc = Arc::new(req);
    let req_clone = Arc::clone(&req_arc);
    let handle = thread::spawn(move || {
        unsafe {
            let trusted = CStr::from_ptr(TRUSTED.as_ptr() as *const i8)
                .to_str()
                .unwrap();
            let header = std::str::from_utf8_unchecked(req_clone.header.as_bytes());
            if header == trusted {
                println!("Info: Origin match detected, but processing anyway.");
            }
        }
    });
    handle.join().unwrap();
    Ok(())
}
fn main() {
    let req = Request {
        header: "api.trusted.com".to_string(),
        body: "Important data".to_string(),
    };
    process_request(req).unwrap();
    println!("Request handled.");
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test contract:
    // When a request comes with a non-trusted origin, the secure version should reject it
    // (return an Err), while the vulnerable version would wrongly accept it (return Ok).
    //
    // Note: This test oracle is designed to be run separately against either the vulnerable or
    // fixed implementation. In a secure implementation, the test should pass; in a vulnerable
    // implementation, the test should fail.
    #[test]
    fn test_origin_validation() {
        // Create a request with a malicious origin
        let req = Request {
            header: "malicious.com".to_string(),
            body: "Attack payload".to_string(),
        };
        // The secure implementation is expected to reject the request.
        let result = process_request(req);
        assert!(
            result.is_err(),
            "Request with unauthorized origin should be rejected."
        );
    }
}
