use std::env;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
const MAX_QUEUE: usize = 5000;
static mut GLOBAL_QUEUE: Option<Vec<String>> = None;

fn init_queue() {
    unsafe {
        if GLOBAL_QUEUE.is_none() {
            GLOBAL_QUEUE = Some(Vec::with_capacity(1024));
        }
    }
}

fn enqueue(item: String) {
    unsafe {
        if let Some(ref mut queue) = GLOBAL_QUEUE {
            if queue.len() < MAX_QUEUE {
                queue.push(item);
            }
        }
    }
}

fn process_queue() {
    loop {
        unsafe {
            if let Some(ref queue) = GLOBAL_QUEUE {
                for s in queue.iter() {
                    let _ = s.as_str(); // Use as_str instead of unsafe pointer dereference
                }
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
}

pub fn run(count: usize) -> usize {
    init_queue();
    thread::spawn(|| {
        process_queue();
    });
    for i in 0..count {
        let task = format!("Task number {}", i);
        unsafe {
            enqueue(task);
        }
    }
    thread::sleep(Duration::from_secs(1));
    unsafe {
        GLOBAL_QUEUE.as_ref().map(|queue| queue.len()).unwrap_or(0)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let count = if args.len() > 1 && args[1] == "trigger" { 10000 } else { 100 };
    let total = run(count);
    println!("Completed processing, total tasks: {}", total);
}

///////////////////////////////////////////////////////////////
// Test Oracle Function
// This test function calls the exported "run" function with a "trigger"
// input (simulating heavy load). It asserts that the total number of tasks
// does not exceed the safe limit (MAX_QUEUE). For the vulnerable version,
// the assertion will fail (total tasks > MAX_QUEUE), while for the fixed version,
// the assertion is satisfied.
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::env;
    // Import the run function from the appropriate binary.
    // In practice, this test is compiled separately from the main application.
    use super::run;

    const SAFE_LIMIT: usize = 5000;

    #[test]
    fn test_resource_limit() {
        // Trigger heavy load by requesting 10,000 tasks.
        let total = run(10000);
        // For the fixed version, total tasks must not exceed SAFE_LIMIT.
        // If total > SAFE_LIMIT, the test will fail, exposing the vulnerability.
        assert!(total <= SAFE_LIMIT, "Resource consumption exceeded safe limit: {}", total);
    }
}
