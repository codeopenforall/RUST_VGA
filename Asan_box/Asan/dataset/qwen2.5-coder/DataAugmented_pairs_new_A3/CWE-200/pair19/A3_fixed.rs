pub struct Secure {
    secret: String,
}

impl Secure {
    pub fn new(secret: String) -> Self {
        Secure { secret }
    }

    pub fn run_task(&self, index: usize) {
        if index >= self.secret.len() {
            panic!("Access error: invalid index {}.", index);
        }
        // Simulate some task using the secret
        unsafe {
            let _byte = *self.secret.as_ptr().offset(index as isize);
        }
    }
}