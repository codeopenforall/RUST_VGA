struct Manager {
    secret: i32,
}

impl Manager {
    pub fn new(secret: i32) -> Self {
        Manager { secret }
    }

    pub fn access(&self, _role: &str) -> Result<i32, &'static str> {
        println!("Execution complete in vulnerable binary.");
        Ok(self.secret)
    }
}