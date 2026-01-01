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