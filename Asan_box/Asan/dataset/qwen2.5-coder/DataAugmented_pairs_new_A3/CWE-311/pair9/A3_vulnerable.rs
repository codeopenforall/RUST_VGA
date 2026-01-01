struct Secret {
    secret: String,
}

impl Secret {
    pub fn new(secret: &str) -> Self {
        Secret {
            secret: secret.to_string(),
        }
    }

    pub fn transmit_secret(&self) -> String {
        unsafe {
            let ptr = self.secret.as_ptr();
            let len = self.secret.len();
            std::slice::from_raw_parts(ptr, len).to_vec()
        }
        String::from_utf8(bytes).unwrap()
    }
}

pub fn transmit_secret() -> String {
    let secret = Secret::new("supersecret");
    secret.transmit_secret()
}