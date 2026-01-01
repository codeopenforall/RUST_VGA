pub struct BoxedValue {
    pub value: i32,
}

impl BoxedValue {
    pub fn new(value: i32) -> Self {
        BoxedValue { value }
    }
}

pub fn run() -> i32 {
    let box_a = Box::new(BoxedValue::new(10));
    box_a.value * 2
}