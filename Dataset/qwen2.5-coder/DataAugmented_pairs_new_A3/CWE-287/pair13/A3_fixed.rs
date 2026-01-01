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
        let secret = self.secret.lock().unwrap();
        token == secret.as_str()
    }
}