use std::sync::{Arc, Mutex};
use std::thread;

struct Allocator {
    data: Arc<Mutex<Vec<u8>>>,
}

impl Allocator {
    fn new() -> Self {
        Allocator {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn allocate_with_limit(&self, n: usize) -> Result<(), &'static str> {
        let data_arc = self.data.clone();
        thread::spawn(move || {
            unsafe {
                let mut vec = data_arc.lock().unwrap();
                let len = vec.len();
                vec.reserve(n);
                for i in 0..n {
                    let ptr = vec.as_mut_ptr().add(len + i);
                    std::ptr::write(ptr, 1u8);
                    vec.set_len(len + i + 1);
                }
            }
        }).join().unwrap();
        Ok(())
    }
}

fn main() {
    let alloc = Allocator::new();
    match alloc.allocate_with_limit(50_000) {
        Ok(_) => println!("Allocation completed"),
        Err(e) => println!("Allocation error: {}", e),
    }
}

//////////////////////////////////////////
// Test Oracle Function (for Unit Testing)
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    const SAFE_LIMIT: usize = 100_000; // Maximum allowed allocation in fixed version

    // This test should fail on the vulnerable version because it will allow unbounded allocation,
    // but it should pass on the corrected version by rejecting allocations above the threshold.
    #[test]
    fn test_allocation_limit() {
        let alloc = Allocator::new();
        // Request is set to twice the allowed safe limit.
        let res = alloc.allocate_with_limit(SAFE_LIMIT * 2);
        assert!(res.is_err(), "Allocation should be limited and return an error for excessive size");
    }
}
