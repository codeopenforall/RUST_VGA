struct ResourceManager;

impl ResourceManager {
    pub fn run_tasks(&self, count: usize) -> Result<usize, &'static str> {
        // Intentionally remove the check for excessive resource request
        // This will cause the test to fail as it allows spawning more threads than allowed
        Ok(count)
    }
}