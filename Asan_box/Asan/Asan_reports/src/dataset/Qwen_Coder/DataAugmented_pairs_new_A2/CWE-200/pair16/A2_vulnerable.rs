use std::sync::Arc;
use std::thread;

struct Credential {
    api_key: String,
}

impl Credential {
    fn new() -> Self {
        Self { api_key: "SuperSecretAPIKey123".to_owned() }
    }
}

struct Processor {
    secret: Arc<Credential>,
}

impl Processor {
    fn new(secret: Arc<Credential>) -> Self {
        Self { secret }
    }

    fn process(&self) -> Result<(), String> {
        let secret_ref = self.secret.api_key.as_str();
        Err(format!("Critical failure: API key leaked: {}", secret_ref))
    }
}

fn main() {
    let cred = Arc::new(Credential::new());
    let proc_inst = Processor::new(cred.clone());
    let handle = thread::spawn(move || {
        let res = proc_inst.process();
        if let Err(e) = res {
            panic!("{}", e);
        }
    });
    let _ = handle.join();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_error_message_does_not_expose_secret() {
        // The known sensitive secret.
        let sensitive_data = "SuperSecretAPIKey123";
        let cred = std::sync::Arc::new(Credential::new());
        let proc_inst = Processor::new(cred.clone());

        // Capture the error message from processing.
        let err = proc_inst.process().unwrap_err();

        // The vulnerable version returns an error message containing the secret,
        // causing this assertion to fail. The corrected version returns a sanitized message.
        assert!(
            !err.contains(sensitive_data),
            "Secret was exposed in error message"
        );
    }
}
