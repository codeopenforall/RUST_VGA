use std::thread;
static mut TASK_COUNT: usize = 0;
struct ResourceService;
impl ResourceService {
    pub fn execute_request(&self, num: usize) -> Result<(), String> {
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
        // Introduce a defect: always return Ok instead of checking a limit
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

////////////////////////////////////////////////////////////
// The following test oracle verifies that excessive task
// requests are properly rejected. When executed against the
// vulnerable implementation, the test fails (since no cap is set),
// but it passes with the fixed implementation where a limit is enforced.
////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_allocation_limit() {
        let service = ResourceService;
        // Request a number of tasks that exceeds the safe limit.
        let result = service.execute_request(1000);
        // The secure behavior is to return an error when the request exceeds the limit.
        assert!(
            result.is_err(),
            "Exceeding the safe limit should be rejected."
        );
    }
}
