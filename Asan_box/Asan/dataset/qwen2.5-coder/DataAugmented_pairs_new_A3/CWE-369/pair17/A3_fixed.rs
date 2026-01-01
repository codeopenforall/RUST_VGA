pub struct Calculator;

pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Calculator {
    pub fn compute(&self, dividend: i32, divisor: i32) -> i32 {
        if divisor == 0 {
            0
        } else {
            unsafe {
                dividend / divisor
            }
        }
    }
}