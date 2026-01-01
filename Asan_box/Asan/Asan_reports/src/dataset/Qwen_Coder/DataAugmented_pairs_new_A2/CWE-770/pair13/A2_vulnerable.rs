use std::thread;
use std::sync::{Arc, Mutex};

struct Executor;
impl Executor {
    fn perform(task_id: usize) {
        unsafe {
            let ptr = Vec::<u8>::with_capacity(1).as_mut_ptr();
            *ptr.offset(0) = (task_id % 256) as u8;
        }
    }
}

fn run_allocation(task_count: usize) -> Result<(), &'static str> {
    let mut threads = Vec::new();
    let shared_counter = Arc::new(Mutex::new(0usize));
    for i in 0..task_count {
        let counter_clone = Arc::clone(&shared_counter);
        let thread_handle = thread::spawn(move || {
            Executor::perform(i);
            let mut counter = counter_clone.lock().unwrap();
            *counter += 1;
        });
        threads.push(thread_handle);
    }
    for thread_handle in threads {
        thread_handle.join().map_err(|_| "Thread panicked")?;
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let task_count: usize = args.get(1).unwrap_or(&"0".to_string()).parse().unwrap_or(0);
    match run_allocation(task_count) {
        Ok(_) => println!("Processing completed"),
        Err(e) => println!("Error encountered: {}", e),
    }
}

#[cfg(test)]
mod tests {
    // Import the run_allocation function from the current module.
    use super::*;

    // The test uses an input that exceeds the allowable threshold.
    // In the fixed version, the function should return an error.
    // In the vulnerable version, it would attempt to spawn more threads,
    // potentially leading to resource exhaustion or unexpected behavior.
    #[test]
    fn test_resource_limit() {
        let excessive_tasks = 1500; // This exceeds THREAD_LIMIT in fixed version.
        let result = run_allocation(excessive_tasks);
        assert!(result.is_err(), "The task count should be capped with an error");
    }
}
