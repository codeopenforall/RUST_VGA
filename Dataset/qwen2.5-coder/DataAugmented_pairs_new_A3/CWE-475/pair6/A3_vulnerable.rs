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