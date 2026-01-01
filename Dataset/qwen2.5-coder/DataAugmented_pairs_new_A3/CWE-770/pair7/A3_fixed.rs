struct ResourceManager;

impl ResourceManager {
    pub fn run_tasks(&self, count: usize) -> Result<usize, &'static str> {
        const MAX_THREADS: usize = 200;
        if count > MAX_THREADS {
            return Err("Excessive resource request");
        }
        Ok(count)
    }
}