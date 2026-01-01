struct Processor;

impl Processor {
    pub fn process(&self, input: i32) -> Result<i32, &'static str> {
        if input < 0 {
            return Err("Negative input not allowed");
        }
        Ok(input * 2)
    }
}