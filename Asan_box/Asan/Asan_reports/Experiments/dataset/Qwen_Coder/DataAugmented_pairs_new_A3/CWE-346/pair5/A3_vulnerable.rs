use std::ffi::CStr;

struct Request {
    origin: String,
    body: String,
}

struct Engine {
    trusted: String,
}

impl Engine {
    pub fn new(trusted: &str) -> Self {
        Engine {
            trusted: trusted.to_string(),
        }
    }

    pub fn process(&self, req: Request) -> bool {
        unsafe {
            let raw = req.origin.as_ptr();
            let parsed = CStr::from_ptr(raw as *const i8)
                .to_string_lossy()
                .into_owned();
            parsed == self.trusted
        }
    }
}

//////////////////////////////////////////////
// Test Oracle Function
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    // The following test should FAIL when run against the vulnerable implementation,
    // since the malicious header "https://trusted.com\0.evil" will be incorrectly validated as allowed.
    // In the fixed implementation, the full string does not match and the request is correctly blocked.
    #[test]
    fn origin_validation_test() {
        let engine = Engine::new("https://trusted.com");
        // Malicious header designed to bypass unsafe C-style conversion.
        let req = Request {
            origin: "https://trusted.com\0.evil".to_string(),
            body: "malicious payload".to_string(),
        };
        let result = engine.process(req);
        // The expected behavior is to block the request, so process() should return false.
        assert!(!result, "Origin header spoofing vulnerability: request should be blocked");
    }
}
//////////////////////////////////////////////
// End of Test Oracle
//////////////////////////////////////////////
