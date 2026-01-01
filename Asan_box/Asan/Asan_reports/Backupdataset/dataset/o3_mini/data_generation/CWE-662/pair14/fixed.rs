use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
struct SharedResource {
    lock_a: Mutex<()>,
    lock_b: Mutex<()>,
    counter: Mutex<u32>,
}
impl SharedResource {
    pub fn new() -> Self {
        Self {
            lock_a: Mutex::new(()),
            lock_b: Mutex::new(()),
            counter: Mutex::new(0),
        }
    }
    pub fn op1(&self) {
        let guard_a = self.lock_a.lock().unwrap();
        unsafe {
            let ptr: *const () = &*guard_a;
            let _dummy = ptr as usize;
        }
        thread::sleep(Duration::from_millis(50));
        let guard_b = self.lock_b.lock().unwrap();
        let mut count = self.counter.lock().unwrap();
        *count += 1;
        drop(guard_b);
        drop(guard_a);
    }
    pub fn op2(&self) {
        let guard_a = self.lock_a.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        let guard_b = self.lock_b.lock().unwrap();
        let mut count = self.counter.lock().unwrap();
        *count += 1;
        drop(guard_b);
        drop(guard_a);
    }
}
fn main() {
    let shared = Arc::new(SharedResource::new());
    let shared1 = Arc::clone(&shared);
    let handle1 = thread::spawn(move || {
        for _ in 0..10 {
            shared1.op1();
        }
    });
    let shared2 = Arc::clone(&shared);
    let handle2 = thread::spawn(move || {
        for _ in 0..10 {
            shared2.op2();
        }
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    let count = shared.counter.lock().unwrap();
    println!("Counter: {}", *count);
}


///////////////////////////////////////////////////////////////
// Test Oracle: This function serves as a unit test ensuring
// the absence (or presence) of deadlock. The test spawns the main
// process in a separate thread and uses a timeout to detect deadlock.
// In the vulnerable version, the test will time out and fail, while it 
// will complete successfully for the corrected version.
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;
    
    // Import main from the current module.
    use super::main;

    #[test]
    fn oracle_test() {
        let (tx, rx) = mpsc::channel();
        // Spawn main's execution in a separate thread.
        thread::spawn(move || {
            main();
            let _ = tx.send(());
        });
        // Wait for completion with a timeout.
        let res = rx.recv_timeout(Duration::from_secs(2));
        assert!(res.is_ok(), "Test failed: Deadlock detected");
    }
}
