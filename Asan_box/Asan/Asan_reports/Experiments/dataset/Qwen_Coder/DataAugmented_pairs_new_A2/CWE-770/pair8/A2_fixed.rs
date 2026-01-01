use std::thread;
const TASK_LIMIT: usize = 100;
struct ResourceHandler;
impl ResourceHandler {
    pub fn execute_tasks(&self, count: usize) -> Result<(), &'static str> {
        if count > TASK_LIMIT {
            return Err("Number of requested tasks exceeds the allowed limit");
        }
        let mut thread_handles = Vec::new();
        for _ in 0..count {
            let handle = thread::spawn(|| {
                let block_size = 1_000_000;
                let mut buffer = vec![0u8; block_size];
                buffer[0] = 42;
            });
            thread_handles.push(handle);
        }
        for th in thread_handles {
            th.join().map_err(|_| "Failed joining thread")?;
        }
        Ok(())
    }
}
fn main() {
    let handler = ResourceHandler;
    let _ = handler.execute_tasks(50);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_limit_enforcement() {
        let handler = ResourceHandler;
        // Provide an input exceeding the allowed threshold.
        let result = handler.execute_tasks(500);
        // Fixed code should return an error, while the vulnerable version would attempt resource allocation.
        assert!(result.is_err(), "Expected error due to too many tasks requested");
    }
}
