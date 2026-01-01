struct Engine;

impl Engine {
    fn new() -> Self {
        Engine
    }

    fn process(&self, input: u32) -> bool {
        let res = external_process(input);
        if res == 0 {
            true
        } else {
            false
        }
    }
}

// Simulated external process that returns 0 on success and non-zero on failure
fn external_process(_input: u32) -> i32 {
    // For demonstration purposes, let's assume any input > 100 returns an error
    if _input > 100 {
        1
    } else {
        0
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
