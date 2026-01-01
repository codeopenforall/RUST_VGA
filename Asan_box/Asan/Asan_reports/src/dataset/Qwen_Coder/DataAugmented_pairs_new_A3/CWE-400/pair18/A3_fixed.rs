use std::sync::{Arc, Mutex};

const MAX_CAPACITY: usize = 1000;

struct Pool {
    data: Mutex<Vec<i32>>,
}

impl Pool {
    pub fn new() -> Self {
        Pool {
            data: Mutex::new(Vec::new()),
        }
    }

    pub fn add(&self, value: i32) {
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
    for i in 0..1500 {
        pool.add(i);
    }
    pool.count() <= MAX_CAPACITY
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
