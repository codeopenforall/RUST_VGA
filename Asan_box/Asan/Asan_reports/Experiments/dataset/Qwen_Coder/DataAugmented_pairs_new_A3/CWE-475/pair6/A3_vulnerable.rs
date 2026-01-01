use std::sync::Mutex;
use std::ptr;

struct DataHolder {
    store: Mutex<Vec<u8>>,
}

impl DataHolder {
    fn new(size: usize) -> Self {
        DataHolder {
            store: Mutex::new(vec![0; size]),
        }
    }

    fn get_ptr(&self) -> *mut u8 {
        self.store.lock().unwrap().as_mut_ptr()
    }
}

trait MemoryOps {
    fn duplicate(&self, size: usize) -> Result<(), &'static str>;
}

impl MemoryOps for DataHolder {
    fn duplicate(&self, size: usize) -> Result<(), &'static str> {
        let ptr = self.get_ptr();
        unsafe {
            ptr::copy(ptr, ptr.offset(0), size);
        }
        Ok(())
    }
}

///////////////////////////////////////////////////////////////////////////////
// Test Oracle:
// This unit test verifies that when an out-of-bound size (20) is provided,
// the duplicate() function must return an error. In the vulnerable implementation,
// no such check exists and the function incorrectly returns Ok(()), causing the 
// test to fail. For the corrected code, the test should pass.
// Note: This test oracle must be compiled and run separately from the main() functions.
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{DataHolder, MemoryOps};

    #[test]
    fn test_invalid_copy_size() {
        let holder = DataHolder::new(10);
        // For a proper implementation, requesting more bytes than allocated should yield an error.
        assert!(holder.duplicate(20).is_err(), "Copy operation should fail for an invalid size");
    }
}
