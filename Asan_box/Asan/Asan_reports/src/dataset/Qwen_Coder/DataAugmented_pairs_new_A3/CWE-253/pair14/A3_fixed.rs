struct Operator {
    value: i32,
}

impl Operator {
    pub fn new(value: i32) -> Self {
        Operator { value }
    }

    pub fn execute(&self) -> bool {
        let ret = external_call(self.value);
        ret == 0
    }
}

// Simulated external function that returns 0 on success and -1 on failure.
fn external_call(value: i32) -> i32 {
    if value > 0 {
        0 // Success
    } else {
        -1 // Failure
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that should succeed with the corrected version and fail with the vulnerable version.
    // For the input value of 5, external_call is designed to return 0 for success.
    // Hence, execute() should return true.
    #[test]
    fn test_operator_execution() {
        let op = Operator::new(5);
        // Assert that the operation indicates success.
        assert!(op.execute(), "Expected the operation to succeed with a valid input");
    }
}
