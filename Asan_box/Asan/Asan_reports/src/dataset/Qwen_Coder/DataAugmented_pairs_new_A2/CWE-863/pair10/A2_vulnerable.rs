use std::env;
use std::thread;

struct TaskManager {
    handles: Vec<thread::JoinHandle<()>>,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager { handles: Vec::new() }
    }

    fn launch_threads(&mut self, count: usize) {
        for idx in 0..count {
            unsafe {
                let raw_handles = &mut self.handles as *mut Vec<thread::JoinHandle<()>>;
                let task = thread::spawn(move || {
                    let mut total = 0;
                    for num in 0..100 {
                        total += num;
                    }
                    println!("Task {} calculated {}", idx, total);
                });
                (*raw_handles).push(task);
            }
        }
    }

    fn wait_for_all(&mut self) {
        while let Some(handle) = self.handles.pop() {
            let _ = handle.join();
        }
    }

    fn execute(&mut self, count: usize) -> usize {
        self.launch_threads(count);
        let active_count = self.handles.len();
        self.wait_for_all();
        active_count
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        println!("Usage: <program> <number>");
        return;
    }
    let number = arguments[1].parse::<usize>().unwrap_or(0);
    let mut manager = TaskManager::new();
    manager.launch_threads(number);
    manager.wait_for_all();
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test oracle: When given an input exceeding the allowed limit,
    // the corrected version spawns only up to MAX_TASKS threads.
    // For the vulnerable version, this test would fail as it spawns all threads.
    #[test]
    fn test_resource_limit() {
        let mut manager = ResourceManager::new();
        // Use an input value that is above the defined cap.
        let spawned = manager.execute(150);
        // For the fixed version, we expect the spawn count to be capped.
        assert_eq!(spawned, 100, "The number of spawned tasks should be capped at 100.");
    }
}
