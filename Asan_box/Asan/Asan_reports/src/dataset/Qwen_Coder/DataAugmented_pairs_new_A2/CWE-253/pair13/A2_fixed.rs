use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

struct Processor {
    src: Arc<Vec<u8>>,
    dst: Arc<Mutex<Vec<u8>>>,
    expected_len: usize,
}

impl Processor {
    fn new(src: Vec<u8>, dst: Vec<u8>) -> Self {
        let expected_len = src.len();
        Processor {
            src: Arc::new(src),
            dst: Arc::new(Mutex::new(dst)),
            expected_len,
        }
    }

    unsafe fn simulated_copy(&self, src_ptr: *const u8, dst_ptr: *mut u8, len: usize) -> i32 {
        if len != self.expected_len {
            return -1;
        }
        ptr::copy_nonoverlapping(src_ptr, dst_ptr, len);
        0
    }

    fn run(&self) -> i32 {
        let src_ptr = self.src.as_ptr();
        let mut dst_lock = self.dst.lock().unwrap();
        let dst_ptr = dst_lock.as_mut_ptr();
        let len = self.src.len();
        let status: i32;
        unsafe {
            status = self.simulated_copy(src_ptr, dst_ptr, len);
        }
        if status == 0 {
            0
        } else {
            -1
        }
    }
}

fn main() {
    let src = vec![1u8, 2, 3, 4];
    let dst = vec![0u8; 4];
    let processor = Processor::new(src, dst);
    let proc_clone = processor;
    let handle = thread::spawn(move || {
        let ret = proc_clone.run();
        std::process::exit(ret);
    });
    handle.join().unwrap();
}

/////////////////////// Test Oracle ///////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    // This test function should be added separately from both main examples.
    // It creates a processor with expected input and asserts that the data transfer
    // operation returns 0 upon success.
    #[test]
    fn test_data_transfer() {
        // Prepare input where the transfer should succeed.
        let src = vec![10u8, 20, 30, 40];
        let dst = vec![0u8; 4];
        let proc_inst = Processor::new(src, dst);
        
        let result = proc_inst.run();
        
        // Assert that a successful transfer returns 0.
        // In the vulnerable version, this assertion fails due to reversed logic.
        assert_eq!(result, 0, "Data transfer did not succeed as expected");
    }
}
