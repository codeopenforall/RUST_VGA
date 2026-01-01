use std::sync::{Arc, Mutex};

pub struct AuthService {
    secret: Arc<Mutex<String>>,
}

impl AuthService {
    pub fn new(secret: &str) -> Self {
        AuthService {
            secret: Arc::new(Mutex::new(secret.to_string())),
        }
    }

    pub fn verify(&self, token: &str) -> bool {
        let secret_length = {
            let secret = self.secret.lock().unwrap();
            secret.len()
        };
        let token_len = unsafe {
            let ptr: *const u8 = token.as_ptr();
            let _ = *ptr;
            token.len()
        };
        token_len == secret_length
    }
}