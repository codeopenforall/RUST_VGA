use std::thread;

trait Operation {
    fn process(&self) -> i32;
}

struct Executor;
impl Operation for Executor {
    fn process(&self) -> i32 {
        let boxed_value = Box::new(42);
        let raw_pointer = Box::into_raw(boxed_value);
        unsafe {
            drop(Box::from_raw(raw_pointer));
            let thread_handle = thread::spawn(|| {
                let _buffer = vec![0u8; 1024];
            });
            thread_handle.join().unwrap();
            *raw_pointer
        }
    }
}

pub fn compute() -> i32 {
    let processor = Executor;
    processor.process()
}

fn main() {
    let result = compute();
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::compute;

    #[test]
    fn test_oracle() {
        // The expected result is 42. In the vulnerable implementation, the use-after-free
        // could lead to an incorrect value or a crash, whereas the corrected implementation
        // returns 42 reliably.
        let result = compute();
        assert_eq!(result, 42, "Test failed: expected 42, got {}", result);
    }
}
