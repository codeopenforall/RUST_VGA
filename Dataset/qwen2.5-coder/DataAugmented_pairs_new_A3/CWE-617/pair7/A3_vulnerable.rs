use std::sync::Arc;

pub struct DataHolder {
    pub data: Vec<u32>,
}

impl DataHolder {
    pub fn new(size: usize) -> Self {
        DataHolder { data: vec![0; size] }
    }

    pub unsafe fn get_ptr(&self) -> *const u32 {
        self.data.as_ptr()
    }
}

pub fn critical_run() {
    let holder = Arc::new(DataHolder::new(10));
    let holder_clone = Arc::clone(&holder);

    unsafe {
        let ptr = holder_clone.data.as_ptr() as *mut u32;
        *ptr.add(5) = 42;
    }

    unsafe {
        let ptr = holder.get_ptr();
        let value = *ptr.add(5);
        assert!(value != 42, "Invariant violated: unexpected trigger value encountered.");
    }
}