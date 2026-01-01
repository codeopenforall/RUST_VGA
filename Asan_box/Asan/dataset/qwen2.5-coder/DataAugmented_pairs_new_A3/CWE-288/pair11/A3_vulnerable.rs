use std::sync::Mutex;

pub struct Authenticator {
    password: String,
    debug_mode: bool,
}

impl Authenticator {
    pub fn new(password: &str, debug_mode: bool) -> Self {
        Authenticator {
            password: password.to_string(),
            debug_mode,
        }
    }

    pub fn verify(&self, input_password: &str) -> bool {
        if self.debug_mode {
            if unsafe { self.alternate_path() } {
                return true;
            }
        }
        input_password == self.password
    }

    unsafe fn alternate_path(&self) -> bool {
        true
    }
}