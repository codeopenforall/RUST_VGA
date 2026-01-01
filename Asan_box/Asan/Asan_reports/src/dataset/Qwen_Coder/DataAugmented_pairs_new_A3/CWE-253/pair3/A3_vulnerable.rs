struct Engine;

impl Engine {
    pub fn new() -> Self {
        Engine
    }

    pub fn process(&self, input: u32) -> bool {
        let res = external_process(input);
        if res != 0 {
            true // Vulnerable: treating non-zero as success
        } else {
            false
        }
    }
}

// Simulated external process that returns 1 for input > 100
fn external_process(input: u32) -> i32 {
    if input > 100 {
        1 // Simulating an error
    } else {
        0 // Simulating success
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation() {
        let engine = Engine::new();
        // For the given triggering input (150), the external_process returns 1 indicating an error.
        // The fixed behavior should return false (indicating failure), while the vulnerable version incorrectly returns true.
        let outcome = engine.process(150);
        assert_eq!(outcome, false, "Expected operation to fail for input > 100");
    }
}
