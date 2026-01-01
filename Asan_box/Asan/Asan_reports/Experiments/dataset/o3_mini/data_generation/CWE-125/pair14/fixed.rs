use std::sync::Arc;
use std::thread;
struct Processor {
    buffer: Vec<u32>,
}
impl Processor {
    fn new() -> Self {
        Processor { buffer: vec![100, 200, 300, 400, 500] }
    }
    fn operate(&self, idx: usize) -> Option<u32> {
        if idx < self.buffer.len() {
            Some(self.buffer[idx])
        } else {
            None
        }
    }
}
fn run_op() -> Option<u32> {
    let proc = Processor::new();
    proc.operate(5)
}
fn main() {
    let shared = Arc::new(Processor::new());
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        run_op()
    });
    match handle.join() {
        Ok(result) => println!("Operation result: {:?}", result),
        Err(_) => println!("Thread panicked"),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_oracle() {
        // The expected behavior of the safe (fixed) version is to return None when an out-of-bound index is used.
        // In the vulnerable version, unsafe access may return Some(_) or even trigger a panic.
        // This test will pass for the fixed code and fail for the vulnerable code.
        let result = panic::catch_unwind(|| {
            run_op()
        });
        // If the function panicked, the test fails.
        if let Err(_) = result {
            panic!("Test failed: function panicked");
        }
        let value = result.unwrap();
        assert!(value.is_none(), "Test failed: Expected None for out-of-bound access");
    }
}
