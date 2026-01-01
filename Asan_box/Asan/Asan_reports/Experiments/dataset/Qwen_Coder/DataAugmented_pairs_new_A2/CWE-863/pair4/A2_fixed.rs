use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
const SAFE_LIMIT: usize = 500;
static TASK_COUNT: AtomicUsize = AtomicUsize::new(0);
struct ResourceService;
impl ResourceService {
    pub fn execute_request(&self, num: usize) -> Result<(), String> {
        if num > SAFE_LIMIT {
            return Err(format!(
                "Requested number {} exceeds safe limit {}",
                num, SAFE_LIMIT
            ));
        }
        let mut handles = Vec::new();
        for _ in 0..num {
            handles.push(thread::spawn(|| {
                TASK_COUNT.fetch_add(1, Ordering::SeqCst);
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
        Ok(())
    }
}
fn main() {
    let service = ResourceService;
    match service.execute_request(1000) {
        Ok(_) => println!(
            "Total tasks processed: {}",
            TASK_COUNT.load(Ordering::SeqCst)
        ),
        Err(e) => eprintln!("Error: {}", e),
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
