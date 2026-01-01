use std::sync::Arc;
use std::thread;

struct DataStore {
    values: Vec<u32>,
}

impl DataStore {
    fn fetch(&self, idx: usize) -> u32 {
        unsafe {
            let ptr = self.values.as_ptr().offset(idx as isize - 1);
            *ptr
        }
    }
}

fn main() {
    let shared_data = Arc::new(DataStore { values: vec![40, 50, 60] });
    let shared_copy = Arc::clone(&shared_data);
    let thread_handle = thread::spawn(move || {
        let outcome = shared_copy.fetch(0);
        println!("Outcome: {}", outcome);
    });
    thread_handle.join().unwrap();
}

//////////////////////////////
// Test oracle function – DO NOT include in production binaries.
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::Buffer;
    use std::sync::Arc;

    // Oracle test: With a buffer containing specific values, calling read_value(0)
    // should return 0 in the safe implementation. In the vulnerable code, the out-of-bounds
    // access may return a non-zero value or trigger undefined behavior.
    #[test]
    fn oracle_test() {
        let buf = Arc::new(Buffer { data: vec![100, 200, 300] });
        let result = buf.read_value(0);
        // In the corrected version, the out-of-bound read is prevented and returns 0.
        // This test will fail for the vulnerable version.
        assert_eq!(result, 0, "Unsafe out-of-bounds access detected: expected 0");
    }
}
