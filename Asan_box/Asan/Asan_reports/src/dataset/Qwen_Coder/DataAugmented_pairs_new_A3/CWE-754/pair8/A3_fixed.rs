pub struct Processor;

impl Processor {
    pub fn perform(&self, status: &str) -> Result<i32, &str> {
        let op_status = match status {
            "success" => OperationStatus::Success,
            "fail" => OperationStatus::Failure,
            "timeout" => OperationStatus::Timeout,
            _ => return Err("Unknown operation status"),
        };

        match op_status {
            OperationStatus::Success => Ok(42),
            OperationStatus::Failure => Err("Operation failed"),
            OperationStatus::Timeout => Err("Operation timed out"),
        }
    }
}

enum OperationStatus {
    Success,
    Failure,
    Timeout,
}

//////////////////////////////
// Test Oracle Function     //
// This test asserts that when "fail" is passed as input,
// the vulnerable implementation incorrectly returns success,
// whereas the fixed implementation properly returns an error.
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::Processor;

    // The test oracle function should be run against both implementations.
    // For the vulnerable version, this test is expected to fail since "fail" is misinterpreted.
    // For the fixed version, it should pass by returning an Err("Operation failed").
    #[test]
    fn oracle_test() {
        let proc = Processor;
        let result = proc.perform("fail");
        // The expected behavior is to get an error with a specific message.
        match result {
            Ok(_) => panic!("Test failed: Expected an error on input \"fail\""),
            Err(e) => assert_eq!(e, "Operation failed", "Unexpected error message"),
        }
    }
}
