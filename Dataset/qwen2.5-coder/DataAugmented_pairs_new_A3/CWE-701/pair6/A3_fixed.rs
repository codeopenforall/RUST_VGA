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