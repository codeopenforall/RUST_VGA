use std::sync::{Arc, Mutex};
use std::thread;

struct Calculator {
    number: u64,
}

impl Calculator {
    fn create(num: u64) -> Self {
        Calculator { number: num }
    }
    unsafe fn reduce(&mut self, dec: u64) {
        let addr = &mut self.number as *mut u64;
        *addr -= dec;
    }
    fn fetch(&self) -> u64 {
        self.number
    }
}

pub fn process(input: u64, sub: u64) -> Result<u64, &'static str> {
    let mut calc = Calculator::create(input);
    unsafe {
        calc.reduce(sub);
    }
    if calc.fetch() > input {
        Err("Underflow detected")
    } else {
        Ok(calc.fetch())
    }
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
