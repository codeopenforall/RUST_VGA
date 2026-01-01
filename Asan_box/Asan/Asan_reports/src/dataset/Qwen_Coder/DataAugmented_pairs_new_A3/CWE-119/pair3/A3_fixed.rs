use std::sync::{Arc, Mutex};

struct Holder {
    data: Vec<i32>,
}

impl Holder {
    fn new(size: usize) -> Self {
        Holder {
            data: vec![0; size],
        }
    }

    fn update(&mut self, index: usize, value: i32) {
        if index < self.data.len() {
            unsafe {
                let ptr = self.data.as_mut_ptr();
                *ptr.add(index) = value;
            }
        } else {
            panic!("Index out of bounds: {} for length {}", index, self.data.len());
        }
    }
}

fn operate(holder: Arc<Mutex<Holder>>, index: usize, value: i32) {
    let mut holder = holder.lock().unwrap();
    holder.update(index, value);
}

////////////////// Test Oracle //////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This test will call operate with an out-of-bound index.
    // In the fixed code, update should panic because of the bound check.
    // In the vulnerable code, undefined behavior (likely no panic) may occur.
    // The test expects a panic; thus, it will pass for the corrected version and fail for the vulnerable one.
    #[test]
    fn test_out_of_bounds() {
        let instance = Arc::new(Mutex::new(Holder::new(10)));
        let result = panic::catch_unwind(|| {
            // Attempt an out-of-bound update
            operate(Arc::clone(&instance), 10, 99);
        });
        assert!(result.is_err(), "Expected panic on out-of-bound update");
    }
}
