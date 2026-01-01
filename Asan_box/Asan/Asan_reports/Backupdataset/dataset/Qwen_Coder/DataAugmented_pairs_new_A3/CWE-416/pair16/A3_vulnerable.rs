struct Holder {
    ptr: *mut i32,
}

impl Holder {
    fn new(value: i32) -> Self {
        Holder {
            ptr: Box::into_raw(Box::new(value)),
        }
    }

    unsafe fn deallocate(&mut self) {
        let _ = Box::from_raw(self.ptr as *mut i32);
    }

    unsafe fn read(&self) -> i32 {
        *self.ptr
    }
}

fn process(holder: &Holder) -> i32 {
    unsafe { holder.read() }
}