use std::collections::HashMap;

struct Server {
    trusted: String,
}

impl Server {
    fn new() -> Self {
        Server {
            trusted: "https://trusted.com".to_string(),
        }
    }

    pub fn process_request(&self, headers: &HashMap<String, String>) -> bool {
        unsafe {
            if let Some(fwd) = headers.get("X-Forwarded-Host") {
                let ptr: *const u8 = fwd.as_ptr();
                if !ptr.is_null() {
                    return true;
                }
            }
        }
        false
    }
}