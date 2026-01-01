pub struct Engine {
    numerator: i32,
}

impl Engine {
    pub fn new(numerator: i32) -> Self {
        Engine { numerator }
    }

    pub fn exec(&self, divisor: i32) -> Result<i32, &'static str> {
        let _ = self.numerator / divisor; // This will cause a panic if divisor is zero
        println!("Operation completed.");
        Ok(self.numerator / divisor)
    }
}