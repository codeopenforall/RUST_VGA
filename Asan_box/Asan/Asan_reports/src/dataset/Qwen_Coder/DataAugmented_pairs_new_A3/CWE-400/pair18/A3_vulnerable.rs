use std::cell::UnsafeCell;
use std::sync::Arc;

struct Pool {
    data: UnsafeCell<Vec<i32>>,
}

unsafe impl Sync for Pool {}

impl Pool {
    fn new() -> Arc<Self> {
        Arc::new(Pool {
            data: UnsafeCell::new(Vec::new()),
        })
    }

    fn add(&self, value: i32) {
        unsafe {
            (*self.data.get()).push(value);
        }
    }

    fn count(&self) -> usize {
        unsafe { (*self.data.get()).len() }
    }
}

fn simulate() -> bool {
    let pool = Pool::new();
    for i in 0..1500 {
        pool.add(i);
    }
    let count = pool.count();
    count <= 1000
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
