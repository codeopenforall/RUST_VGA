struct Authenticator {
    secret: String,
}

impl Authenticator {
    pub fn new(secret: &str) -> Self {
        Authenticator {
            secret: secret.to_string(),
        }
    }

    pub fn process(&self, input: &str) -> bool {
        unsafe {
            let secret_bytes = self.secret.as_bytes();
            let input_bytes = input.as_bytes();
            let len = if secret_bytes.len() < input_bytes.len() { 
                secret_bytes.len() 
            } else { 
                input_bytes.len() 
            };
            for i in 0..len {
                if *input_bytes.get_unchecked(i) != secret_bytes[i] {
                    return false;
                }
            }
            true
        }
    }
}

fn process(input: &str) -> bool {
    let authenticator = Authenticator::new("verysecurekey");
    authenticator.process(input)
}

/////////////////////////////
// Test Oracle Functionality
/////////////////////////////
#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn oracle() {
        // This input is only a prefix ("very") of the valid secret "verysecurekey".
        // Expect authentication to be rejected.
        let test_token = "very";
        let result = process(test_token);
        assert!(!result, "Test failed: An incomplete token was incorrectly accepted");
    }
}
