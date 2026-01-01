struct Manager {
    secret: i32,
}

impl Manager {
    pub fn new(secret: i32) -> Self {
        Manager { secret }
    }

    pub fn access(&self, role: &str) -> Result<i32, &'static str> {
        if role != "admin" {
            return Err("Not authorized");
        }
        println!("Execution complete in secure binary.");
        Ok(self.secret)
    }
}