use std::sync::Arc;
use std::thread;

trait Multiply {
    fn multiply(&self, a: u32, b: u32) -> Result<u32, &'static str>;
}

struct Calculator;
impl Multiply for Calculator {
    fn multiply(&self, a: u32, b: u32) -> Result<u32, &'static str> {
        let result = a.checked_mul(b).ok_or("Multiplication overflowed")?;
        Ok(result)
    }
}

fn main() {
    let calculator = Arc::new(Calculator);
    let handle = {
        let calc_clone = Arc::clone(&calculator);
        thread::spawn(move || {
            let a = std::u32::MAX;
            let b = 2;
            calc_clone.multiply(a, b)
        })
    };
    match handle.join().unwrap() {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overflow() {
        let calc = Calculator;
        let a = std::u32::MAX;
        let b = 2;
        let result = calc.multiply(a, b);
        assert!(result.is_err(), "Overflow was not detected: {:?}", result);
    }
}