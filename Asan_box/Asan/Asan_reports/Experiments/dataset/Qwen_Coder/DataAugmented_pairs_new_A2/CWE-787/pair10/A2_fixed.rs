use std::sync::{Arc, Mutex};
use std::thread;

struct DataPool {
    buffer: Vec<u8>,
}

trait Modifier {
    fn modify(&mut self, index: usize, value: u8) -> Result<(), &'static str>;
}

impl Modifier for DataPool {
    fn modify(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        if index < self.buffer.len() {
            self.buffer[index] = value;
            Ok(())
        } else {
            Err("Index out-of-bounds")
        }
    }
}

impl DataPool {
    fn new(size: usize) -> Self {
        let buffer = vec![0u8; size];
        DataPool { buffer }
    }
}

fn main() {
    let pool = Arc::new(Mutex::new(DataPool::new(10)));
    let pool_clone = Arc::clone(&pool);
    let handle = thread::spawn(move || {
        let mut dp = pool_clone.lock().unwrap();
        dp.modify(5, 42).expect("In-bound modification must succeed");
    });
    handle.join().unwrap();
    let mut dp = pool.lock().unwrap();
    match dp.modify(11, 99) {
        Ok(_) => println!("Unexpectedly modified out-of-bound index"),
        Err(e) => println!("Failed modification as expected: {}", e),
    }
    println!("Final buffer: {:?}", dp.buffer);
}

////////////////////////////////////////////////////////////
// Test Oracle:
// This test checks the behavior of the Modifier::modify method.
// A valid in-bound modification should succeed, while an out-of-bound
// modification should return an error. In the vulnerable version, the
// out-of-bound call improperly succeeds (leading to memory corruption),
// whereas in the fixed version, the call fails as expected.
////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::{DataPool, Modifier};

    #[test]
    fn test_bounds_check() {
        // Create a DataPool of size 10.
        let mut dp = DataPool::new(10);

        // In-bound modification should work.
        assert!(dp.modify(5, 100).is_ok(), "In-bound modification must succeed");

        // Out-of-bound modification should be rejected.
        let result = dp.modify(11, 200);
        assert!(result.is_err(), "Out-of-bound modification should return an error");
    }
}
