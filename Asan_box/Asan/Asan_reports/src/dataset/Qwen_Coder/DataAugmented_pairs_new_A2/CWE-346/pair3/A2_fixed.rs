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
        let trusted = CStr::from_bytes_with_nul(TRUSTED)
            .map_err(|_| "Invalid trusted string")?
            .to_str()
            .map_err(|_| "Invalid UTF-8 in trusted string")?;
        let header = req_clone.header.as_str();
        if header != trusted {
            return Err("Unauthorized origin");
        }
        Ok(())
    });
    let result = handle.join().unwrap();
    result
}
fn main() {
    let req = Request {
        header: "api.trusted.com".to_string(),
        body: "Important data".to_string(),
    };
    process_request(req).unwrap();
    println!("Request handled securely.");
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
