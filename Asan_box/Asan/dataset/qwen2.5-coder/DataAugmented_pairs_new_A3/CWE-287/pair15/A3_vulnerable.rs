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