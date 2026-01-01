struct Authenticator {
    valid: String,
}

impl Authenticator {
    pub fn check(&self, token: &str) -> bool {
        unsafe {
            let valid_bytes = self.valid.as_bytes();
            let token_bytes = token.as_bytes();
            let len = token_bytes.len();
            if len > valid_bytes.len() {
                return false;
            }
            let p_valid = valid_bytes.as_ptr();
            let slice_valid = std::slice::from_raw_parts(p_valid, len);
            slice_valid == token_bytes
        }
    }
}

fn main() {
    let locked = Authenticator {
        valid: "supersecrettoken".to_string(),
    };
    println!("Authentication result: {}", locked.check("supersecret"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_auth() {
        // Using the same struct as in both versions.
        let auth = Authenticator {
            valid: "supersecrettoken".to_string(),
        };
        // Exact match should pass.
        assert_eq!(auth.check("supersecrettoken"), true, "Exact match must pass");
        // A prefix token should fail in the secure (fixed) version.
        assert_eq!(auth.check("supersecret"), false, "Prefix token should not pass authentication");
    }
}
