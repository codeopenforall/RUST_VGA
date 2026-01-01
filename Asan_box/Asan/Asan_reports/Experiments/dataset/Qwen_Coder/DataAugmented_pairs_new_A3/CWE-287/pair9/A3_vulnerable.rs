pub struct AuthManager {
    secret_bytes: Vec<u8>,
}

impl AuthManager {
    pub fn new() -> Self {
        AuthManager {
            secret_bytes: b"verysecret".to_vec(),
        }
    }

    pub fn check(&self, candidate_ptr: *const u8, len: usize) -> bool {
        let candidate = unsafe { std::slice::from_raw_parts(candidate_ptr, len) };
        if len <= self.secret_bytes.len() && candidate == &self.secret_bytes[..len] {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AuthManager;

    // Test oracle: uses a partial token "very".
    // For the vulnerable implementation, this test would pass (erroneously authenticating),
    // but for the fixed implementation, it correctly denies access.
    #[test]
    fn test_authentication() {
        let auth = AuthManager::new();
        let token = "very".as_bytes();
        let result = auth.check(token.as_ptr(), token.len());
        assert_eq!(result, false, "Authentication should fail for a partial token");
    }
}
