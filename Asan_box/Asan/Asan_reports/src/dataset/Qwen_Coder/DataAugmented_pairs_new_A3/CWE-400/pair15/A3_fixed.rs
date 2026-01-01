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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle() {
        // Using a count that exceeds the safe limit.
        let excessive = 2000;
        let srv = Server::new();
        let res = srv.run(excessive);
        assert!(res.is_err(), "Expected rejection for task count above limit");
    }
}
