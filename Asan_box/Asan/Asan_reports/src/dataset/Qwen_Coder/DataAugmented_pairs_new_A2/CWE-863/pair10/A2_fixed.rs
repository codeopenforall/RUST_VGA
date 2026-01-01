use std::env;
use std::thread;
const MAX_TASKS: usize = 100;
struct ResourceManager {
    pool: Vec<thread::JoinHandle<()>>,
}
impl ResourceManager {
    fn new() -> Self {
        ResourceManager { pool: Vec::new() }
    }
    fn spawn_tasks(&mut self, n: usize) {
        let limit = if n > MAX_TASKS { MAX_TASKS } else { n };
        for i in 0..limit {
            let handle = thread::spawn(move || {
                let mut sum = 0;
                for j in 0..100 {
                    sum += j;
                }
                println!("Thread {} computed {}", i, sum);
            });
            self.pool.push(handle);
        }
    }
    fn join_all(&mut self) {
        while let Some(handle) = self.pool.pop() {
            let _ = handle.join();
        }
    }
    fn execute(&mut self, n: usize) -> usize {
        self.spawn_tasks(n);
        let count = self.pool.len();
        self.join_all();
        count
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: <program> <num>");
        return;
    }
    let num = args[1].parse::<usize>().unwrap_or(0);
    let mut manager = ResourceManager::new();
    manager.spawn_tasks(num);
    manager.join_all();
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
