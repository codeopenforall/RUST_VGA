struct Calculator;

impl Calculator {
    pub fn new() -> Self {
        Calculator
    }

    pub fn compute(&self, a: i32, b: i32) -> i32 {
        if b == 0 {
            return 0;
        }
        a / b
    }
}