use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
struct Calculator {
    factor: i32,
}

impl Calculator {
    fn new() -> Self {
        Calculator { factor: 2 }
    }

    fn compute(&self, base: i32, input: u32) -> i32 {
        unsafe {
            let addition = (input as i32) * self.factor;
            base + addition
        }
    }
}

fn main() {
    let calc = Calculator::new();
    let balance = Arc::new(Mutex::new(10_i32));
    let calc = Arc::new(calc);
    let input_val: u32 = 0xFFFF_FFFF;
    let mut handles = vec![];

    for _ in 0..2 {
        let bal_clone = Arc::clone(&balance);
        let calc_clone = Arc::clone(&calc);
        let in_val = input_val;
        let handle = thread::spawn(move || {
            let mut num = bal_clone.lock().unwrap();
            *num = calc_clone.compute(*num, in_val);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *balance.lock().unwrap());
}

--------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    // This test oracle function uses a known large input value.
    // For the safe (fixed) version, the conversion substitutes i32::MAX.
    // The saturating multiplication: i32::MAX.saturating_mul(2) remains i32::MAX.
    // Then base (10) saturating added with i32::MAX still yields i32::MAX.
    // Thus, the expected result is i32::MAX.
    // In the unsafe version, the conversion yields -1, and the result becomes 8.
    #[test]
    fn test_oracle() {
        let calc = Calculator::new();
        let base = 10;
        let input: u32 = 0xFFFF_FFFF;
        let result = calc.compute(base, input);
        assert_eq!(result, i32::MAX, "Safe arithmetic result should saturate to i32::MAX");
    }
}
--------------------------------------------------
