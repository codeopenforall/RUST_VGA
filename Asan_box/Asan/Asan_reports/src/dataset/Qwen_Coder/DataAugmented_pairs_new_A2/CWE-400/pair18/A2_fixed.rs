use std::sync::{Arc, Mutex};
use std::thread;
const MAX_CAPACITY: usize = 1000;
pub struct Pool {
    data: Mutex<Vec<i32>>,
}
impl Pool {
    pub fn new() -> Self {
        Pool {
            data: Mutex::new(Vec::with_capacity(MAX_CAPACITY)),
        }
    }
    pub fn insert(&self, value: i32) {
        let mut vec = self.data.lock().unwrap();
        if vec.len() < MAX_CAPACITY {
            vec.push(value);
        }
    }
    pub fn count(&self) -> usize {
        let vec = self.data.lock().unwrap();
        vec.len()
    }
}
pub fn simulate() -> bool {
    let pool = Arc::new(Pool::new());
    let mut handles = Vec::new();
    for _ in 0..4 {
        let pool_clone = Arc::clone(&pool);
        let handle = thread::spawn(move || {
            for i in 0..1000 {
                pool_clone.insert(i);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let count = pool.count();
    count <= MAX_CAPACITY
}
fn main() {
    let safe = simulate();
    if safe {
        println!("Resource usage within safe limits: {}", safe);
    } else {
        println!("Resource usage exceeded safe limits: {}", safe);
    }
}

////////////////////////////////////////////////////////////
// Test Oracle: Resource Limitation Verification
// This test is designed to validate the resource consumption behavior.
// For the vulnerable implementation, the simulation will result in a resource count
// that exceeds safe thresholds, causing the test to fail. The fixed version will enforce
// a capacity limit and allow the test to pass.
//
// To use this oracle, compile the target code (vulnerable or fixed) as a library,
// then run this test to verify whether the implementation is susceptible to unbounded
// resource consumption.
////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::simulate;

    #[test]
    fn check_resource_limits() {
        // The safe threshold is 1000 elements.
        // In the vulnerable version, simulate() returns false (resource count > 1000).
        // In the fixed version, simulate() returns true (resource count <= 1000).
        let result = simulate();
        assert!(result, "Resource consumption exceeded safe limits");
    }
}
