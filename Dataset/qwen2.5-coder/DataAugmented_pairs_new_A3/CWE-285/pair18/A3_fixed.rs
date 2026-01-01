struct Resource {
    secret: i32,
}

impl Resource {
    unsafe fn get_data(&self, token: &str) -> Result<i32, &'static str> {
        if token != "admin" {
            return Err("Unauthorized");
        }
        Ok(self.secret)
    }
}