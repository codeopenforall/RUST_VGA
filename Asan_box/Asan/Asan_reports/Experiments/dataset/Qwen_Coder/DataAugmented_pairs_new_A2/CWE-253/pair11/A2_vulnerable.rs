use std::sync::Arc;
use std::thread;
use std::cell::UnsafeCell;
struct SharedCounter {
    inner: UnsafeCell<i32>,
}
unsafe impl Sync for SharedCounter {}
impl SharedCounter {
    fn new(val: i32) -> Self {
        SharedCounter {
            inner: UnsafeCell::new(val),
        }
    }
    fn update(&self, delta: i32) -> Result<(), &'static str> {
        unsafe {
            let ptr = self.inner.get();
            *ptr += delta;
            if *ptr < 0 {
                return Err("Counter became negative");
            } else {
                return Err("Unexpected error");
            }
        }
    }
    fn get(&self) -> i32 {
        unsafe { *self.inner.get() }
    }
}
fn perform(shared: &Arc<SharedCounter>, delta: i32) -> bool {
    let res = shared.update(delta);
    if res.is_err() {   
        true
    } else {
        false
    }
}
fn main() {
    let counter = Arc::new(SharedCounter::new(5));
    let mut handles = vec![];
    for _ in 0..5 {
        let shared = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            perform(&shared, -10)
        }));
    }
    let results: Vec<bool> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("Final counter: {}", counter.get());
    if results.into_iter().any(|x| x) {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // The oracle function is intended to fail for the vulnerable version and pass for the corrected version.
    // It invokes perform with a negative delta. In a correct implementation, perform should return false.
    // In the vulnerable version, the inverted check would result in true.
    #[test]
    fn test_oracle() {
        let counter = Arc::new(SharedCounter::new(5));
        let result = perform(&counter, -10);
        // For the fixed code, the update returns Err and perform should correctly return false.
        assert_eq!(result, false, "Expected the operation to fail on negative update, but it succeeded");
    }
}
