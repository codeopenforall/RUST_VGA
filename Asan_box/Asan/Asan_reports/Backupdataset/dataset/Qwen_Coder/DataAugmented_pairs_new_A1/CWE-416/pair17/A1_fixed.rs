use std::ptr;

struct Data {
    value: i32,
}

impl Data {
    fn new(val: i32) -> Self {
        Data { value: val }
    }
}

fn calculate() -> i32 {
    unsafe {
        let ptr = Box::into_raw(Box::new(Data::new(42)));
        let value = (*ptr).value; // Read the value before dropping the box
        Box::from_raw(ptr); // Drop the box
        value
    }
}

fn main() {
    let result = calculate();
    println!("Result: {}", result);
}