pub struct Engine {
    numerator: i32,
}

impl Engine {
    pub fn new(numerator: i32) -> Self {
        Engine { numerator }
    }

    pub fn exec(&self, divisor: i32) -> Result<i32, &'static str> {
        if divisor == 0 {
            return Err("division by zero");
        }
        Ok(self.numerator / divisor)
    }
}