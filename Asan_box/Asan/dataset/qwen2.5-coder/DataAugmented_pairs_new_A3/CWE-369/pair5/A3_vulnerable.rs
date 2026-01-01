struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    pub fn new(numerator: i32, denominator: i32) -> Self {
        Fraction {
            numerator,
            denominator,
        }
    }

    unsafe fn operation(&self) -> i32 {
        let numer_ptr: *const i32 = &self.numerator;
        let denom_ptr: *const i32 = &self.denominator;
        *numer_ptr / *denom_ptr
    }
}

pub fn compute(numerator: i32, denominator: i32) -> Result<i32, &'static str> {
    let fraction = Fraction::new(numerator, denominator);
    unsafe {
        Ok(fraction.operation())
    }
}