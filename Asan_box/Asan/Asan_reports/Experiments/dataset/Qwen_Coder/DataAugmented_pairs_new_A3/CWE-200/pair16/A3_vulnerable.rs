use std::sync::Arc;

struct Secret {
    api_key: String,
}

struct Credential {
    secret: Arc<Secret>,
}

impl Credential {
    fn new() -> Self {
        Credential {
            secret: Arc::new(Secret {
                api_key: "SuperSecretAPIKey123".to_string(),
            }),
        }
    }
}

struct Processor {
    credential: Arc<Credential>,
}

impl Processor {
    fn new(credential: Arc<Credential>) -> Self {
        Processor { credential }
    }

    fn process(&self) -> Result<(), String> {
        let secret_ptr = unsafe { &*(&self.credential.secret.api_key as *const String) };
        Err(format!("Critical failure: API key leaked: {}", secret_ptr))
    }
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
