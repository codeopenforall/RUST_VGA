use std::env;
use std::thread;
use std::sync::mpsc::{channel, Sender};
const SIZE: usize = 10 * 1024 * 1024; 

struct Worker {
    transmitter: Sender<Vec<u8>>,
}

impl Worker {
    fn initialize(transmitter: Sender<Vec<u8>>) -> Self {
        Worker { transmitter }
    }

    fn operate(&self) {
        let transmitter_clone = self.transmitter.clone();
        thread::spawn(move || {
            let mut buffer = Vec::with_capacity(SIZE);
            unsafe {
                buffer.set_len(SIZE);
            }
            transmitter_clone.send(buffer).unwrap();
        });
    }
}

fn dispatch_tasks(task_count: usize) {
    let (tx, rx) = channel();
    let worker = Worker::initialize(tx);
    for _ in 0..task_count {
        worker.operate();
    }
    for _ in 0..task_count {
        let _ = rx.recv().unwrap();
    }
}

#[no_mangle]
pub fn run_app(tasks: usize) -> Result<(), &'static str> {
    dispatch_tasks(tasks);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let job_count: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    run_app(job_count).unwrap();
    println!("Processing finished");
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
