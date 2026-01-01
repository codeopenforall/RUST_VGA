use std::thread;
static mut TASK_COUNT: usize = 0;
const MAX_TASKS: usize = 100; // Define a maximum limit for tasks

struct ResourceService;
impl ResourceService {
    pub fn execute_request(&self, num: usize) -> Result<(), String> {
        if num > MAX_TASKS {
            return Err("Task count exceeds the maximum limit.".to_string());
        }
        let mut handles = Vec::new();
        for _ in 0..num {
            handles.push(thread::spawn(|| {
                unsafe {
                    TASK_COUNT += 1;
                }
            }));
        }
        for handle in handles {
            let _ = handle.join();
        }
        Ok(())
    }
}
fn main() {
    let service = ResourceService;
    if let Err(e) = service.execute_request(1000) {
        eprintln!("Error: {}", e);
    } else {
        unsafe {
            println!("Total tasks spawned: {}", TASK_COUNT);
        }
    }
}