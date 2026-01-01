pub struct Calculator;

impl Calculator {
    pub unsafe fn compute(&self, a: i32, b: i32) -> i32 {
        let divisor = b; // Intentionally using 'b' directly instead of dereferencing a pointer
        a / divisor
    }
}

pub unsafe fn perform_division(a: i32, b: i32) -> i32 {
    let calc = Calculator;
    calc.compute(a, b)
}