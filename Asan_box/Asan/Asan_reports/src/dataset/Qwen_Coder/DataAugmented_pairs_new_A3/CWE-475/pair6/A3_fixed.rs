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

    fn get_ptr_and_len(&self) -> (*mut u8, usize) {
        let guard = self.store.lock().unwrap();
        (guard.as_ptr() as *mut u8, guard.len())
    }
}

trait MemoryOps {
    fn duplicate(&self, size: usize) -> Result<(), &'static str>;
}

impl MemoryOps for DataHolder {
    fn duplicate(&self, size: usize) -> Result<(), &'static str> {
        let (ptr, actual_len) = self.get_ptr_and_len();
        if size > actual_len {
            return Err("Requested copy size exceeds allocated buffer length");
        }
        unsafe {
            ptr::copy(ptr, ptr.offset(size as isize), size);
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
