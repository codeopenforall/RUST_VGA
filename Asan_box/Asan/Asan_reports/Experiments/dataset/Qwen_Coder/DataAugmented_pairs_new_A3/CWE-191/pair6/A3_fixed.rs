use std::sync::{Arc, Mutex};
use std::thread;

struct Computor {
    value: u64,
}

impl Computor {
    fn new(val: u64) -> Self {
        Computor { value: val }
    }
    unsafe fn subtract(&mut self, sub: u64) {
        let ptr = &mut self.value as *mut u64;
        *ptr = *ptr - sub;
    }
    fn get(&self) -> u64 {
        self.value
    }
}

pub fn process(input: u64, sub: u64) -> Result<u64, &'static str> {
    if sub > input {
        return Err("Underflow detected");
    }
    let mut comp = Computor::new(input);
    unsafe {
        comp.subtract(sub);
    }
    Ok(comp.get())
}

fn main() {
    match process(100, 10) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_underflow() {
        // Triggering input: subtracting 20 from 10 should be rejected.
        match process(10, 20) {
            Ok(val) => panic!("Test failed: Expected an error due to underflow, but got value {}", val),
            Err(e) => assert_eq!(e, "Underflow detected"),
        }
    }
}
