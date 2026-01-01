use std::sync::Arc;

struct Processor {
    base: i32,
}

impl Processor {
    fn new(base: i32) -> Self {
        Processor { base }
    }

    fn operate(&self, input: &str) -> Result<i32, String> {
        let divisor: i32 = input
            .parse()
            .map_err(|_| "Parsing the divisor failed".to_string())?;
        if divisor == 0 {
            return Err("Divisor cannot be zero".to_string());
        }
        Ok(self.base / divisor)
    }
}

pub fn perform(input: &str) -> Result<i32, String> {
    let processor = Arc::new(Processor::new(100));
    let processor = Arc::clone(&processor);
    processor.operate(input)
}