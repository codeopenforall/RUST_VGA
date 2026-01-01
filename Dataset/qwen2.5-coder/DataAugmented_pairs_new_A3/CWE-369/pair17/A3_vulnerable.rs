pub struct Calculator;

pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Calculator {
    pub fn compute(&self, dividend: i32, divisor: i32) -> i32 {
        unsafe {
            dividend / divisor
        }
    }
}