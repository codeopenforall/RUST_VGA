use std::sync::Arc;

pub struct IdentityManager {
    secret: Arc<String>,
}

impl IdentityManager {
    pub fn new(secret: String) -> Self {
        IdentityManager {
            secret: Arc::new(secret),
        }
    }

    pub fn validate(&self, input: &str) -> bool {
        unsafe {
            if input.is_empty() || (*self.secret).is_empty() {
                return false;
            }
            let input_ptr = input.as_ptr();
            let secret_ptr = self.secret.as_ptr();
            *input_ptr == *secret_ptr
        }
    }
}