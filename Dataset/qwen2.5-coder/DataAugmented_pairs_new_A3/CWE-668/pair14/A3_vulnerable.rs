struct Calculator;

impl Calculator {
    pub fn new() -> Self {
        Calculator
    }

    pub fn compute(&self, a: i32, b: i32) -> i32 {
        if b == 0 {
            panic!("Division by zero");
        }
        a / b
    }
}