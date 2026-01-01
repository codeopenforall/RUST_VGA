struct Data {
    value: i32,
}

struct Handler {
    ptr: *const Data,
}

impl Handler {
    pub fn get_value(&self) -> i32 {
        unsafe {
            (*self.ptr).value  
        }
    }
}

pub fn compute() -> i32 {
    let data_box = Box::new(Data { value: 42 });
    let raw = Box::into_raw(data_box);
    let handler = Handler { ptr: raw };
    unsafe {
        let _freed = Box::from_raw(raw);
    }
    unsafe {
        let dummy = Box::new(Data { value: 0 });
        std::mem::forget(dummy);
    }
    handler.get_value()
}