use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
struct Shared {
    counter: i32,
}
impl Shared {
    fn new() -> Self {
        Shared { counter: 0 }
    }
    fn increment(&mut self) {
        unsafe {
            let ptr: *mut i32 = &mut self.counter;
            *ptr = (*ptr).wrapping_add(1);
        }
    }
    fn value(&self) -> i32 {
        self.counter
    }
}
fn run() {
    let data = Arc::new(Mutex::new(Shared::new()));
    let data_clone = Arc::clone(&data);
    let thr1 = thread::spawn(move || {
        let mut locked = data_clone.lock().unwrap(); 
        locked.increment();
        panic!("Simulated panic to poison the lock");
    });
    thread::sleep(Duration::from_millis(50));
    let data_clone2 = Arc::clone(&data);
    let thr2 = thread::spawn(move || {
        let mut locked = data_clone2.lock().unwrap(); 
        locked.increment();
    });
    let _ = thr1.join();
    let _ = thr2.join();
    let locked = data.lock().unwrap(); 
    println!("Final counter: {}", locked.value());
}
fn main() {
    run();
}


//////////////////////////////////////////
// Test Oracle Function for CWE-662 Example
// This single test invokes the run() function and checks that it
// completes without panicking. For the vulnerable version, the test
// will catch a panic (due to unhandled poison) and fail. For the fixed
// version, run() should complete normally, and the test passes.
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::run;
    use std::panic;

    #[test]
    fn test_oracle() {
        // The expected behavior is that run() completes without panicking.
        let result = panic::catch_unwind(|| {
            run();
        });
        assert!(result.is_ok(), "The run() function panicked unexpectedly");
    }
}
