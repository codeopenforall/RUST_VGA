use std::sync::Arc;
use std::thread;

struct Processor {
    numerator: i32,
    denominator: i32,
}

impl Processor {
    fn operation(&self) -> Result<i32, &'static str> {
        if self.denominator == 0 {
            return Err("Division by zero error");
        }
        Ok(self.numerator / self.denominator)
    }
}

pub fn compute(n1: i32, n2: i32) -> Result<i32, &'static str> {
    let proc = Processor {
        numerator: n1,
        denominator: n2,
    };
    let shared = Arc::new(proc);
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        shared_clone.operation()
    });
    match handle.join() {
        Ok(inner) => inner,
        Err(_) => Err("Thread error"),
    }
}

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| "0".to_string());
    let denom = arg.parse::<i32>().unwrap_or(0);
    match compute(100, denom) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::compute;

    #[test]
    fn division_oracle() {
        // Test input that triggers division by zero.
        let result = compute(100, 0);
        // Fixed version is expected to return a controlled error.
        // Vulnerable version returns a different error message.
        assert_eq!(result, Err("Division by zero error"), "Expected controlled error on division by zero.");
    }
}
