use std::sync::{Mutex, Arc};
use std::thread;
struct Buffer {
    data: Vec<u32>,
}
impl Buffer {
    fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        unsafe {
            data.set_len(size);
        }
        Buffer { data }
    }
    unsafe fn store(&mut self, index: usize, value: u32) {
        *self.data.get_unchecked_mut(index) = value;
    }
}
trait Summable {
    fn total(&self) -> u32;
}
struct Worker {
    buffer: Buffer,
}
impl Summable for Worker {
    fn total(&self) -> u32 {
        self.buffer.data.iter().sum()
    }
}
impl Worker {
    fn new(size: usize) -> Self {
        Worker { buffer: Buffer::new(size) }
    }
    fn run(&mut self) {
        unsafe {
            self.buffer.store(self.buffer.data.len(), 100);
        }
    }
}
fn main() {
    let worker = Arc::new(Mutex::new(Worker::new(5)));
    let worker_clone = Arc::clone(&worker);
    let handle = thread::spawn(move || {
        let mut w = worker_clone.lock().unwrap();
        w.run();
    });
    handle.join().unwrap();
    let total = worker.lock().unwrap().total();
    println!("Computed total: {}", total);
}


///////////////////////////////////////////////////////////////
// Test Oracle: This function serves as a regression test.
// It asserts that a Worker, after running its operation, produces the expected sum.
// For the vulnerable version, invoking the operation leads to an out-of-bounds write,
// and the computed sum will not be as expected (or may even trigger a crash). For the
// fixed version, the sum should be exactly 100.
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::sync::{Mutex, Arc};
    use std::thread;
    
    // The Worker, Buffer, and related trait definitions should be imported
    // from the relevant module.

    // Dummy implementations for testing purposes.
    // Replace these with the implementations from the compiled version.
    use super::{Worker, Summable};

    // Oracle test: For correct behavior, Worker::run should update the buffer
    // and the computed total should equal 100.
    #[test]
    fn oracle_test() {
        // Initialize Worker with a buffer of size 5.
        let worker = Worker::new(5);
        let worker = Arc::new(Mutex::new(worker));
        let worker_clone = Arc::clone(&worker);
    
        let handle = thread::spawn(move || {
            let mut w = worker_clone.lock().unwrap();
            w.run();
        });
    
        handle.join().unwrap();
    
        let total = worker.lock().unwrap().total();
    
        // The expected total is 100 if a valid index was written.
        // In the vulnerable version, the total may be unpredictable.
        assert_eq!(total, 100, "Oracle check failed: total is not equal to 100");
    }
}
