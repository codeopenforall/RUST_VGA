use std::thread;
use std::sync::{Arc, Mutex};
struct Data {
    buf: Vec<u8>,
}
impl Data {
    pub fn new(size: usize) -> Self {
        Data {
            buf: Vec::with_capacity(size),
        }
    }
    pub unsafe fn load(&mut self, input: &[u8]) {
        let capacity = self.buf.capacity();
        let count = input.len();
        let ptr = self.buf.as_mut_ptr();
        for i in 0..count {
            *ptr.add(i) = input[i];
        }
        self.buf.set_len(count);
    }
    pub fn read(&self) -> &[u8] {
        &self.buf
    }
}
fn run(input: Vec<u8>) -> Result<Arc<Mutex<Data>>, &'static str> {
    let shared = Arc::new(Mutex::new(Data::new(10)));
    let shared_clone = Arc::clone(&shared);
    let handler = thread::spawn(move || {
        let mut data = shared_clone.lock().unwrap();
        unsafe {
            data.load(&input);
        }
    });
    if handler.join().is_err() {
        return Err("Thread panicked");
    }
    Ok(shared)
}
fn main() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let res = run(input);
    match res {
        Ok(shared) => {
            let data = shared.lock().unwrap();
            println!("Buffer content: {:?}", data.read());
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}


#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;

    // Declaration of the external function signature.
    // It is assumed that each binary (vulnerable and fixed) provides a "run" function
    // with the signature: fn run(input: Vec<u8>) -> Result<Arc<Mutex<Data>>, &'static str>
    // where Data exposes a method "read()" returning a &[u8].
    //
    // For this oracle, we expect that, when provided with an input that exceeds the capacity,
    // the safe implementation returns an error, whereas the unsafe one would incorrectly succeed.
    // Here, we test the contract that the function must reject inputs that are too long.
    //
    // Note: This test oracle must be compiled and run separately from the vulnerable and fixed binaries.
    // It does not contain any code from the vulnerable or fixed examples.

    #[test]
    fn test_buffer_capacity_guard() {
        // Triggering input: 12 bytes where capacity is fixed at 10.
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

        // Call the "run" function from the implementation under test.
        // The contract in the fixed version is that an input exceeding capacity yields an error.
        let result = super::run(input);

        // If the implementation is fixed, it should return Err.
        // In the vulnerable version, the out-of-bound write may corrupt memory and yield Ok.
        assert!(
            result.is_err(),
            "Contract violation: Expected an error due to input size exceeding capacity."
        );
    }
}
