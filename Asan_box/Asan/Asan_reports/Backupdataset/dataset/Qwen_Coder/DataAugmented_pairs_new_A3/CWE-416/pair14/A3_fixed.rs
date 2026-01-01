pub struct Data {
    pub value: i32,
}

pub fn process() -> i32 {
    let boxed_data = Box::new(Data { value: 42 });
    let raw_ptr = Box::into_raw(boxed_data);
    let result = unsafe { (*raw_ptr).value };
    let _ = unsafe { Box::from_raw(raw_ptr) };
    result
}