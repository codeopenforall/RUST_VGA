pub struct Memory {
    ptr: *mut i32,
}

impl Memory {
    pub fn new(value: i32) -> Self {
        let boxed_value = Box::new(value);
        Memory {
            ptr: Box::into_raw(boxed_value),
        }
    }

    pub fn get(&self) -> i32 {
        unsafe { *self.ptr }
    }
}

pub fn run() -> i32 {
    let mem = Memory::new(256);
    let value = unsafe { mem.get() };
    value
}