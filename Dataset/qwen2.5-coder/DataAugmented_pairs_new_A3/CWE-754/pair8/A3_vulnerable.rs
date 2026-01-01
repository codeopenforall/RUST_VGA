use std::fmt;

#[derive(Debug, PartialEq)]
enum OperationStatus {
    Success,
    Failure,
    Timeout,
}

impl fmt::Display for OperationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct Processor;

impl Processor {
    pub fn perform(&self, input: &str) -> Result<i32, &'static str> {
        let op_status = match input {
            "success" => OperationStatus::Success,
            "fail" => OperationStatus::Failure,
            "timeout" => OperationStatus::Timeout,
            _ => OperationStatus::Failure,
        };

        if op_status != OperationStatus::Timeout {
            Ok(42)
        } else {
            Err("Operation timed out")
        }
    }
}