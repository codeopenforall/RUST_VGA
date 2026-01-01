struct Server;

impl Server {
    pub fn new() -> Self {
        Server
    }

    pub fn run(&self, count: usize) -> Result<(), &'static str> {
        const MAX_LIMIT: usize = 1000;
        if count > MAX_LIMIT {
            return Err("Count exceeds maximum allowed limit");
        }
        Ok(())
    }
}