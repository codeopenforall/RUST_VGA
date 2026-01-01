struct Processor;

impl Processor {
    pub fn heavy_work(&self, tasks: usize) -> Result<(), String> {
        const MAX_TASKS: usize = 1000;
        if tasks > MAX_TASKS {
            return Err("Number of tasks exceeds allowed limit".to_string());
        }
        // Simulate heavy work
        Ok(())
    }
}