use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
struct SharedData {
    count: UnsafeCell<i32>,
}
unsafe impl Sync for SharedData {} 
impl SharedData {
    fn new(val: i32) -> Self {
        SharedData {
            count: UnsafeCell::new(val),
        }
    }
    fn increment(&self) {
        unsafe {
            let old = *self.count.get();
            thread::yield_now(); 
            *self.count.get() = old + 1;
        }
    }
    fn get(&self) -> i32 {
        unsafe { *self.count.get() }
    }
}
fn run_logic() -> i32 {
    let shared = Arc::new(SharedData::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let s = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                s.increment();
            }
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    shared.get()
}
fn main() {
    let final_val = run_logic();
    println!("Final count: {}", final_val);
}


//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::run_logic;

    #[test]
    fn test_concurrent_count() {
        // Expected final count: 10 threads * 1000 increments = 10000.
        let final_val = run_logic();
        assert_eq!(final_val, 10000, "The final count should be 10000.");
    }
}
