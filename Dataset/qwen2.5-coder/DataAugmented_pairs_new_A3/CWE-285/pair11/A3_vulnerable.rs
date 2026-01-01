struct Resource {
    secret: String,
}

impl Resource {
    pub fn new(secret: &str) -> Self {
        Resource {
            secret: secret.to_string(),
        }
    }

    pub unsafe fn obtain_secret(&self, _req: &str) -> Result<&str, &'static str> {
        unsafe {
            match self.secret.as_str() {
                "TopSecret" => Ok(self.secret.as_str()),
                _ => Err("Access denied"),
            }
        }
    }
}