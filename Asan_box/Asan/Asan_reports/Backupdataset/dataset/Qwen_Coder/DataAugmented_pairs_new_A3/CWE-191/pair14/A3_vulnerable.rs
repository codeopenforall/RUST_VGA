pub struct Counter {
    current: u32,
}

impl Counter {
    pub fn new(initial: u32) -> Self {
        Counter { current: initial }
    }

    pub fn decrease(&self, sub: u32) -> u32 {
        let new_val = self.current.wrapping_sub(sub);
        new_val
    }
}

pub fn process_operation(initial: u32, subtractor: u32) -> Result<u32, &'static str> {
    let counter = Counter::new(initial);
    let result = counter.decrease(subtractor);
    Ok(result)
}

fn main() {
    // Example usage
    let initial = 100u32;
    let subtractor = 200u32;
    match process_operation(initial, subtractor) {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}