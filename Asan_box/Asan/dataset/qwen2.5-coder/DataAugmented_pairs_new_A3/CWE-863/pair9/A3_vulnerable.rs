struct Processor;

impl Processor {
    pub fn heavy_work(&self, tasks: usize) -> Result<(), String> {
        // Intentionally omitting the check for MAX_TASKS
        // This makes the code vulnerable to CWE-863: Resource Exhaustion
        // as it allows an unbounded number of tasks to be processed.
        
        // Simulate processing tasks
        for _ in 0..tasks {
            // Task processing logic here
        }
        
        Ok(())
    }
}