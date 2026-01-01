use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
static SAFE_LIMIT: usize = 1000;
trait Worker {
    fn execute(&self);
}
struct Task {
    data: usize,
}
impl Worker for Task {
    fn execute(&self) {
        thread::sleep(Duration::from_millis(1));
    }
}
struct ResourceManager {
    tasks: Arc<Mutex<Vec<thread::JoinHandle<()>>>>,
}
impl ResourceManager {
    fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }
    fn allocate(&self, count: usize) -> Result<(), String> {
        if count > SAFE_LIMIT {
            return Err(format!("Requested tasks {} exceed limit of {}", count, SAFE_LIMIT));
        }
        for i in 0..count {
            let task = Task { data: i };
            let handle = thread::spawn(move || {
                task.execute();
            });
            self.tasks.lock().unwrap().push(handle);
        }
        Ok(())
    }
    fn join_all(&self) {
        let mut lock = self.tasks.lock().unwrap();
        while let Some(handle) = lock.pop() {
            let _ = handle.join();
        }
    }
}
fn main() {
    let manager = ResourceManager::new();
    match manager.allocate(10) {
        Ok(_) => {
            manager.join_all();
            println!("Execution completed in secure build.");
        },
        Err(e) => {
            println!("Allocation error: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_allocation_cap_enforcement() {
        let manager = ResourceManager::new();
        // This value exceeds the permitted limit of 1000.
        let result = manager.allocate(1500);
        // In the secure version, we expect an error to be returned.
        // In the vulnerable version, the error is not raised (and tasks are allocated).
        assert!(result.is_err(), "Allocation should fail when exceeding the safe limit");
        if let Err(msg) = result {
            assert_eq!(msg, "Requested tasks 1500 exceed limit of 1000");
        }
    }
}
