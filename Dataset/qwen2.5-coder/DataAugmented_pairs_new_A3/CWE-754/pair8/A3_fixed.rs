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