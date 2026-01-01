use std::sync::Arc;
use std::thread;

struct Calculator {
    numerator: i32,
    denominator: i32,
}

impl Calculator {
    fn compute(&self) -> i32 {
        unsafe {
            let num_ref: &i32 = &self.numerator;
            let den_ref: &i32 = &self.denominator;
            *num_ref / *den_ref
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let den: i32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let calc = Arc::new(Calculator {
        numerator: 100,
        denominator: den,
    });
    let calc_clone = Arc::clone(&calc);
    let handle = thread::spawn(move || {
        let result = calc_clone.compute();
        println!("Computed result: {}", result);
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::Calculator;
    #[test]
    fn contract_test() {
        // Create an instance with a zero denominator.
        let calc = Calculator { numerator: 100, denominator: 0 };
        // Using catch_unwind to safely capture panics.
        let result = std::panic::catch_unwind(|| {
            // We do not care about the returned value; we only want to ensure that no panic occurs.
            let _ = calc.compute();
        });
        // In the fixed version, compute() returns None without panicking.
        // In the vulnerable version, the division by zero will cause a panic, making is_ok() false.
        assert!(result.is_ok(), "Division by zero was not handled safely.");
    }
}
