use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
const MAX_TASKS: usize = 1000;
struct Service {
    tasks: Arc<Mutex<Vec<String>>>,
}
impl Service {
    fn new() -> Self {
        Service {
            tasks: Arc::new(Mutex::new(Vec::with_capacity(1024))),
        }
    }
    fn submit(&self, data: String) -> Result<(), String> {
        let mut list = self.tasks.lock().map_err(|_| "Lock poisoned".to_string())?;
        if list.len() >= MAX_TASKS {
            return Err("Queue limit reached".to_string());
        }
        list.push(data);
        Ok(())
    }
}
fn main() {
    let svc = Service::new();
    {
        let tasks_clone = svc.tasks.clone();
        thread::spawn(move || {
            loop {
                {
                    let mut list = tasks_clone.lock().unwrap();
                    if !list.is_empty() {
                        list.remove(0);
                    }
                }
                thread::sleep(Duration::from_millis(50));
            }
        });
    }
    for i in 0..1100 {
        if let Err(e) = svc.submit(format!("Task {}", i)) {
            eprintln!("Submission error: {}", e);
            break;
        }
    }
    println!("Tasks submission complete (max limit enforced)");
}

///////////////////////////////////////////////////////////////////////////////
// Test Oracle: This function is used in a unit test environment. It creates an 
// instance of the service and attempts to submit 1100 tasks. In the fixed version, 
// once the queue limit (MAX_TASKS) is reached, submit() returns an error and the 
// test asserts that error detection occurred. In the vulnerable version, all tasks 
// are accepted, causing the test to fail.
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_limit() {
        let svc = Service::new();
        let mut error_detected = false;
        // Try to submit slightly above the maximum limit.
        for i in 0..1100 {
            if svc.submit(format!("Task {}", i)).is_err() {
                error_detected = true;
                break;
            }
        }
        assert!(error_detected, "Expected rejection when queue limit is exceeded");
    }
}
