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