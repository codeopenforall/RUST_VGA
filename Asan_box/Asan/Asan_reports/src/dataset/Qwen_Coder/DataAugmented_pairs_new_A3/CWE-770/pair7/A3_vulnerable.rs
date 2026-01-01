struct ResourceManager;

impl ResourceManager {
    pub fn run_tasks(&self, count: usize) -> Result<usize, &'static str> {
        // Intentionally remove the check for excessive resource request
        // This will cause the test to fail as it allows spawning more threads than allowed
        Ok(count)
    }
}

/////////////////////////////////////////////////////////////////
// Test Oracle: This function is executed in a unit test environment.
// It calls the resource allocation function with an input that exceeds the secure limit.
// For the insecure version, the function would spawn 250 threads and return Ok(250),
// causing the test to fail. For the secure version, the function should return an error.
/////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_throttling() {
        let manager = ResourceManager;
        // Use an input that exceeds the allowed maximum.
        let result = manager.run_tasks(250);
        match result {
            Ok(n) => {
                // In the vulnerable version, spawning 250 threads leads to success,
                // which is not acceptable.
                panic!("Test failed: Expected error due to excessive resource request, but got {} tasks", n);
            },
            Err(e) => {
                // In the secure version, an error is rightly returned.
                assert_eq!(e, "Excessive resource request");
            }
        }
    }
}
