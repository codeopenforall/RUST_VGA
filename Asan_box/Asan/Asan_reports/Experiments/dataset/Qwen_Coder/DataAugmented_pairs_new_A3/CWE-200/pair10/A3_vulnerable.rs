use std::fmt;

#[derive(Debug)]
struct Credential {
    secret: String,
}

impl fmt::Display for Credential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Credential {{ secret: {} }}", self.secret)
    }
}

struct Processor {
    credential: Credential,
}

impl Processor {
    fn new(credential: Credential) -> Self {
        Processor { credential }
    }

    fn process(&self) -> Result<(), String> {
        unsafe {
            let ptr = self.credential.secret.as_ptr();
            let len = self.credential.secret.len();
            let slice = std::slice::from_raw_parts(ptr, len);
            let secret_str = std::str::from_utf8(slice).unwrap();
            Err(format!("Processing error with credential: {:?}", secret_str))?
        }
    }
}

fn main() {
    let cred = Credential { secret: "super_secret_1234".to_string() };
    let proc = Processor::new(cred);
    match proc.process() {
        Ok(_) => println!("Processing succeeded"),
        Err(e) => eprintln!("Error occurred: {:?}", e),
    }
}

/*
The following test oracle ensures that the sensitive token is not leaked in error messages.
It should fail when run against the vulnerable code (because the secret is exposed)
and pass when run against the fixed code (since the secret is redacted).

To run the test, include this in your test suite (e.g., in a module with #[cfg(test)]).
*/

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_error_message() {
        let cred = Credential { secret: "super_secret_1234".to_string() };
        let proc = Processor::new(cred);
        let result = proc.process();
        assert!(result.is_err(), "Expected an error due to processing failure");
        let err_msg = format!("{}", result.unwrap_err());
        // This assertion fails in the vulnerable version (where the secret is leaked)
        // and passes in the fixed version (where the secret is redacted).
        assert!(
            !err_msg.contains("super_secret_1234"),
            "Secret leaked in error message!"
        );
    }
}
