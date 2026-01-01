use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Record {
    value: i32,
}

impl Record {
    fn new(val: i32) -> Self {
        Record { value: val }
    }
}

unsafe fn compute() -> i32 {
    let boxed = Box::new(Record::new(42));
    let ptr = Box::into_raw(boxed);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        let _temp = Box::new(Record::new(100));
    });
    handle.join().unwrap();
    unsafe {
        let record = Box::from_raw(ptr);
        record.value
    }
}

fn main() {
    let result = unsafe { compute() };
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    // For the vulnerable implementation, this test is expected to fail because compute() returns a corrupted value.
    // For the corrected implementation, compute() returns the correct value 42, and the test passes.
    use super::*;
    
    #[test]
    fn test_compute_value() {
        // For the vulnerable code, compute() is unsafe and may trigger undefined behavior.
        // The expected invariant is that compute() should return 42.
        let result = unsafe { compute() };
        assert_eq!(result, 42, "Test failed: expected 42 but got {}", result);
    }
}
