use std::fmt;

struct SecureData {
    secret: String,
}

impl SecureData {
    pub fn new(secret: String) -> Self {
        SecureData { secret }
    }

    pub fn process(&self, value: i32) -> Result<(), String> {
        if value > 100 {
            let exposed = unsafe {
                let ptr: *const String = &self.secret;
                (*ptr).clone()
            };
            return Err(format!(
                "Invalid value: {}. Debug info: {:?}",
                value, exposed
            ));
        }
        Ok(())
    }
}

impl fmt::Debug for SecureData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SecureData {{ secret: {} }}", self.secret)
    }
}