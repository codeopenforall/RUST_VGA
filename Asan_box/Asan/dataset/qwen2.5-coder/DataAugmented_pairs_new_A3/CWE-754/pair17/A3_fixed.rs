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