pub struct Memory {
    pub ptr: *mut i32,
}

impl Memory {
    pub fn new(value: i32) -> Self {
        let boxed = Box::new(value);
        Memory {
            ptr: Box::into_raw(boxed),
        }
    }

    pub unsafe fn get(&self) -> i32 {
        *self.ptr
    }
}

pub fn run() -> i32 {
    let mem = Memory::new(256);
    let raw = mem.ptr;
    unsafe {
        *raw = 9999;
        *raw
    }
}