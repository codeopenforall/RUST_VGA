use std::error::Error;

#[derive(Debug)]
enum ProcError {
    Timeout,
    Other(String),
}

impl Error for ProcError {}

impl std::fmt::Display for ProcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcError::Timeout => write!(f, "Process timed out"),
            ProcError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

struct Worker;

impl Worker {
    fn new() -> Self {
        Worker
    }

    fn execute(&self, input: i32) -> Result<i32, ProcError> {
        if input % 2 != 0 {
            Err(ProcError::Timeout)
        } else {
            Ok(input * 2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exceptional_condition() {
        let worker = Worker::new();
        // For input 1 (an odd number), the process is designed to time out.
        // In the fixed version, execute returns an error.
        // In the vulnerable version, it erroneously returns a default value (100).
        let result = worker.execute(1);
        match result {
            Err(ProcError::Timeout) => assert!(true, "Proper error propagated"),
            Ok(val) => panic!("Test failed: Expected error, but got success with value: {}", val),
            _ => panic!("Test failed: Unexpected result variant"),
        }
    }
}
