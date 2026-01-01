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