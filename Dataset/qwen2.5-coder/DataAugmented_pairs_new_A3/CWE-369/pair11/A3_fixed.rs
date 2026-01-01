struct Calculator;

impl Calculator {
    unsafe fn transform(&self, a: i32, b: i32) -> Result<i32, &'static str> {
        if b == 0 {
            return Err("Division by zero error");
        }
        Ok(a / b)
    }
}

fn compute(a: i32, b: i32) -> Result<i32, &'static str> {
    let calc = Calculator;
    unsafe { calc.transform(a, b) }
}

pub fn run_calc(a: i32, b: i32) -> Result<i32, &'static str> {
    compute(a, b)
}