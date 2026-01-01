struct Division {
    numerator: i32,
    denominator: i32,
}

impl Division {
    pub fn operation(&self) -> Result<i32, &'static str> {
        if self.denominator == 0 {
            return Err("Division by zero error");
        }
        unsafe {
            let numer_ptr: *const i32 = &self.numerator;
            let denom_ptr: *const i32 = &self.denominator;
            Ok(*numer_ptr / *denom_ptr)
        }
    }
}

pub fn compute(numerator: i32, denominator: i32) -> Result<i32, &'static str> {
    let division = Division { numerator, denominator };
    division.operation()
}