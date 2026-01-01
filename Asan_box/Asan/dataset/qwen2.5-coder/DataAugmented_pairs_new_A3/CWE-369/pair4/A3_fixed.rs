pub struct Calculator;

impl Calculator {
    pub unsafe fn compute(&self, a: i32, b: i32) -> Result<i32, &'static str> {
        if b == 0 {
            return Err("Division by zero");
        }
        let divisor = b; // Assuming b_ptr was a pointer to b, which is unnecessary here
        Ok(a / divisor)
    }
}

pub unsafe fn perform_division(a: i32, b: i32) -> i32 {
    let calc = Calculator;
    match calc.compute(a, b) {
        Ok(val) => val,
        Err(err) => {
            println!("Error encountered: {}", err);
            0
        }
    }
}