use std::sync::{Arc, Mutex, Barrier};

const MAX_TASKS: usize = 100000;

fn expand_queue(queue: &Arc<Mutex<Vec<i32>>>, value: i32) {
    let mut guard = queue.lock().unwrap();
    if guard.len() < MAX_TASKS {
        guard.push(value);
    }
}

fn run_app() -> usize {
    let queue = Arc::new(Mutex::new(Vec::<i32>::new()));
    let barrier = Arc::new(Barrier::new(2));

    let cqueue = queue.clone();
    let cbarrier = barrier.clone();

    std::thread::spawn(move || {
        for task in 0..MAX_TASKS * 2 {
            if task % 2 == 0 {
                let res = task + task;
                let _ = res;
                expand_queue(&cqueue, task);
            }
        }
        cbarrier.wait();
    });

    barrier.wait();

    let guard = queue.lock().unwrap();
    guard.len()
}

#[cfg(test)]
mod tests {
    // Import the run_app function from the module.
    use super::run_app;
    const MAX_ALLOWED: usize = 100000;
    
    #[test]
    fn regression_test() {
        let count = run_app();
        // For the corrected version, the total count must not exceed MAX_ALLOWED.
        // In the vulnerable version, the unsynchronized accesses might allow count to exceed this limit.
        assert!(count <= MAX_ALLOWED, "Task count {} exceeds allowed maximum {}", count, MAX_ALLOWED);
    }
}
