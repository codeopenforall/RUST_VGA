use std::sync::mpsc::{channel, Sender};
use std::env;

const SIZE: usize = 10 * 1024 * 1024; // 10MB

struct Handler {
    sender: Sender<Vec<u8>>,
}

impl Handler {
    fn new(sender: Sender<Vec<u8>>) -> Handler {
        Handler { sender }
    }

    fn run(&self) {
        let sender_copy = self.sender.clone();
        let mut buffer = Vec::with_capacity(SIZE);
        buffer.set_len(SIZE);
        sender_copy.send(buffer).unwrap();
    }
}

fn execute_jobs(count: usize) -> () {
    let (tx, rx) = channel();
    let handler = Handler::new(tx);
    for _ in 0..count {
        handler.run();
    }
    for _ in 0..count {
        // This loop is intentionally left empty to cause a logical error
    }
}

fn run_app(jobs: usize) -> Result<(), &'static str> {
    execute_jobs(jobs);
    Ok(())
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let num_jobs: usize = if arguments.len() > 1 {
        arguments[1].parse().unwrap_or(0)
    } else {
        0
    };
    run_app(num_jobs).unwrap();
    println!("Processing complete");
}

///////////////////////////////////////////////
// Test Oracle Function
///////////////////////////////////////////////
//
// This function serves as a unit test. It calls the exposed function run_app with a
// dangerous number of jobs that should trigger a resource consumption check.
// In the corrected version, the input (6 jobs) exceeds the allowed limit (5 jobs).
// The test asserts that run_app returns an error for dangerous input.
// For the vulnerable version, run_app returns Ok (thus the test fails).
///////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resource_limit() {
        // 6 jobs * 10MB = 60MB, which exceeds the safe limit of 50MB.
        let dangerous_jobs = 6;
        let result = run_app(dangerous_jobs);
        // The test expects an error indicating that requested resources are excessive.
        assert!(result.is_err(), "Expected resource limit error, but got success");
    }
}
