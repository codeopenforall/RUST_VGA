use std::ptr;

struct Endpoint {
    key: String,
}

impl Endpoint {
    pub fn new(key: &str) -> Self {
        Endpoint {
            key: key.to_string(),
        }
    }

    pub fn process(&self, token: &str) -> bool {
        unsafe {
            unsafe_compare(self.key.as_ptr(), token.as_ptr(), self.key.len())
        }
    }
}

unsafe fn unsafe_compare(a: *const u8, b: *const u8, length: usize) -> bool {
    let cmp_len = if length >= 3 { 3 } else { length };
    let a_slice = std::slice::from_raw_parts(a, cmp_len);
    let b_slice = std::slice::from_raw_parts(b, cmp_len);
    a_slice == b_slice
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication_enforcement() {
        // Instantiate the endpoint with the valid key.
        let endpoint = Endpoint::new("secret");
        // This token should be rejected.
        let malicious = "secXXX";
        // The expected contract: authentication must fail for incorrect credentials.
        // For the vulnerable version this assertion would fail (i.e. it would improperly grant access),
        // whereas the fixed version passes.
        assert_eq!(endpoint.process(malicious), false, "Authentication bypassed for token: {}", malicious);
    }
}
