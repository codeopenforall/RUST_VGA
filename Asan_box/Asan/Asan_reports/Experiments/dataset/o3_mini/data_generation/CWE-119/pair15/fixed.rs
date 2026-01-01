use std::sync::{Arc, Mutex};
use std::thread;
trait Operations {
    fn update(&self, index: usize, value: i32);
}
struct Buffer {
    data: Mutex<Vec<i32>>,
}
impl Buffer {
    fn new(size: usize) -> Self {
        Self {
            data: Mutex::new(vec![0; size]),
        }
    }
}
impl Operations for Buffer {
    fn update(&self, index: usize, value: i32) {
        let mut guard = self.data.lock().unwrap();
        if index < guard.len() {
            unsafe {
                let ptr = guard.as_mut_ptr();
                *ptr.add(index) = value;
            }
        } else {
            panic!("Attempt to write outside of buffer bounds");
        }
    }
}
fn main() {
    let buf = Arc::new(Buffer::new(10));
    let handles: Vec<_> = (0..2)
        .map(|_| {
            let b = Arc::clone(&buf);
            thread::spawn(move || {
                b.update(15, 99);
            })
        })
        .collect();
    for h in handles {
        h.join().expect("Thread panicked as expected due to invalid access");
    }
    let guard = buf.data.lock().unwrap();
    println!("Buffer contents: {:?}", *guard);
}


///////////////////////////////////////////////////////////////
// This test oracle function is used to verify the behavior of the 
// implementations. It is designed to ensure that an out-of-bounds
// update operation results in a panic (which is the correct and safe
// behavior in the fixed version). The vulnerable version (which lacks
// bounds check) is expected to not panic and thus will cause this test
// to fail.
///////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn oracle() {
        // Create a buffer with 10 elements.
        let buf = Buffer::new(10);
        // Use catch_unwind to evaluate if the update operation panics.
        let result = panic::catch_unwind(|| {
            // Attempting an update with an invalid index.
            // The fixed implementation should panic.
            buf.update(15, 99);
        });
        assert!(result.is_err(), "Update did not panic on out-of-bounds access");
    }
}
