struct Data {
    value: i32,
}

impl Data {
    fn new(value: i32) -> Self {
        Data { value }
    }
}

pub fn compute() -> i32 {
    let boxed = Box::new(Data::new(42));
    let raw_ptr = Box::into_raw(boxed);
    unsafe {
        Box::from_raw(raw_ptr);
        let _dummy = Box::new(Data::new(100));
        (*raw_ptr).value
    }
}