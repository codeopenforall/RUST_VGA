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

#[cfg(test)]
mod tests {
    use super::transmit_secret;

    #[test]
    fn test_encrypted_transmission() {
        let transmitted = transmit_secret();
        // In the vulnerable version, the secret is sent in clear text ("supersecret").
        // In the fixed version, it is encrypted and encoded as hex.
        // This test fails if the transmitted data equals the clear text.
        assert_ne!(transmitted, "supersecret", "Sensitive data transmitted in clear text!");
    }
}
