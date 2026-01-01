pub struct Secure {
    secret: String,
}

impl Secure {
    pub fn new(secret: String) -> Self {
        Secure { secret }
    }

    pub fn run_task(&self, index: usize) {
        unsafe {
            let _byte = *self.secret.as_ptr().offset(index as isize);
            panic!("Access error: secret leak: {}. Index: {}", self.secret, index);
        }
    }
}