use std::sync::Arc;
use std::thread;

struct Calculator;

impl Calculator {
    fn compute(&self, numerator: i32, denominator: i32) -> Result<i32, String> {
        if denominator == 0 {
            return Err(String::from("Division by zero error"));
        }
        Ok(numerator / denominator)
    }

    pub fn safe_compute(&self, numerator: i32, denominator: i32) -> Result<i32, String> {
        self.compute(numerator, denominator)
    }
}

fn main() {
    let calc = Arc::new(Calculator);
    let calc_ref = Arc::clone(&calc);
    let handle = thread::spawn(move || {
        calc_ref.compute(100, 0)
    });
    match handle.join().unwrap() {
        Ok(value) => println!("Computed result: {}", value),
        Err(err) => println!("Encountered error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::Calculator;

    #[test]
    fn division_by_zero_oracle() {
        let calc = Calculator;
        // Using the unified API wrapper safe_compute, which in the fixed version returns Err("Division by zero error")
        // whereas in the vulnerable version, a panic is caught and translated to Err("Panic occurred").
        let result = calc.safe_compute(10, 0);
        match result {
            Ok(val) => panic!("Expected an error when dividing by zero, but got Ok({})", val),
            Err(ref msg) if msg == "Division by zero error" => (), // Correct behavior in the fixed version.
            Err(ref msg) => panic!("Unexpected error message: {}", msg),
        }
    }
}
