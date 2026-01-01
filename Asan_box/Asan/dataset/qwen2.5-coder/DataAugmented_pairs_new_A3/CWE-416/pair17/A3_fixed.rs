struct Data {
    value: i32,
}

impl Data {
    fn new(value: i32) -> Self {
        Data { value }
    }
}

pub fn calculate() -> i32 {
    let boxed = Box::new(Data::new(42));
    let ptr = Box::into_raw(boxed);
    let result = unsafe { (*ptr).value };
    // Ensure the memory is properly freed
    drop(unsafe { Box::from_raw(ptr) });
    result
}