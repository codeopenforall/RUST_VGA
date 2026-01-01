use std::sync::Arc;
use std::thread;

struct DataStore {
    values: Vec<u8>,
}

impl DataStore {
    fn create() -> Self {
        DataStore { values: vec![1, 2, 3, 4, 5] }
    }
    pub fn retrieve(&self, index: usize) -> u8 {
        unsafe { *self.values.get_unchecked(index) }
    }
}

pub fn execute() -> u8 {
    let store = Arc::new(DataStore::create());
    let cloned_store = Arc::clone(&store);
    let thread_handle = thread::spawn(move || {
        cloned_store.retrieve(10)
    });
    thread_handle.join().unwrap()
}

fn main() {
    let result = execute();
    println!("Result: {}", result);
}

//////////////////////////////////////////////////////////////
// Test Oracle: This unit test is intended to verify the behavior
// of the code. When run against the vulnerable version, the test
// should fail (due to a panic or undefined behavior caused by an
// out-of-bounds read). When run against the fixed version, the test
// should pass, returning the expected value (3). This single test
// ensures the contract of safe, bounds-checked memory access.
//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::execute;

    #[test]
    fn test_boundary() {
        // For the fixed version, execute() returns the element at index 2,
        // which is 3 in the array [1, 2, 3, 4, 5].
        //
        // In the vulnerable version, execute() invokes an unsafe out-of-bound
        // access (index 10) which is expected to panic or produce undefined behavior.
        //
        // This test passes when execute() returns 3, and fails (panic or
        // assertion failure) if it does not.
        let res = execute();
        assert_eq!(res, 3, "Expected result 3 from safe access but got a different value");
    }
}
