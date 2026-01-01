use std::env;
use std::thread;
use std::sync::mpsc::{channel, Sender};
const SIZE: usize = 10 * 1024 * 1024; 
const MAX_TOTAL: usize = 50 * 1024 * 1024; 
struct Processor {
    sender: Sender<Vec<u8>>,
}
impl Processor {
    fn new(sender: Sender<Vec<u8>>) -> Self {
        Processor { sender }
    }
    fn dispatch(&self) {
        let sender_clone = self.sender.clone();
        thread::spawn(move || {
            let mut data = Vec::with_capacity(SIZE);
            unsafe {
                data.set_len(SIZE);
            }
            sender_clone.send(data).unwrap();
        });
    }
}
fn process_jobs(job_count: usize) -> Result<(), &'static str> {
    if job_count * SIZE > MAX_TOTAL {
        return Err("Requested resource exceeds safe limits");
    }
    let (tx, rx) = channel();
    let processor = Processor::new(tx);
    for _ in 0..job_count {
        processor.dispatch();
    }
    for _ in 0..job_count {
        let _ = rx.recv().unwrap();
    }
    Ok(())
}
#[no_mangle]
pub fn run_app(jobs: usize) -> Result<(), &'static str> {
    process_jobs(jobs)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let num_jobs: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    match run_app(num_jobs) {
        Ok(_) => println!("Processing complete"),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
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
