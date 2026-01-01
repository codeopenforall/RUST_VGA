use std::thread;
use std::sync::Arc;

struct Calculator {
    factor: u64,
}

impl Calculator {
    fn compute(&self, data: &[u32]) -> u64 {
        let mut sum: u64 = 0;
        for &value in data {
            sum = sum.wrapping_add(value as u64);
        }
        sum.checked_mul(self.factor).expect("Multiplication overflow")
    }
}

fn run_calc() -> u64 {
    let numbers = Arc::new(vec![u32::MAX, 1]);
    let calc = Arc::new(Calculator { factor: 2 });
    let numbers_clone = Arc::clone(&numbers);
    let calc_clone = Arc::clone(&calc);
    let handle1 = thread::spawn(move || {
        calc_clone.compute(&numbers_clone[0..1])
    });
    let numbers_clone2 = Arc::clone(&numbers);
    let calc_clone2 = Arc::clone(&calc);
    let handle2 = thread::spawn(move || {
        calc_clone2.compute(&numbers_clone2[1..2])
    });
    let part1 = handle1.join().unwrap();
    let part2 = handle2.join().unwrap();
    part1.checked_add(part2).expect("Addition overflow")
}

fn main() {
    let result = run_calc();
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_oracle() {
        // The correct result should be:
        // (u32::MAX as u64 + 1) * 2 = (4294967295 + 1)*2 = 8589934592.
        let expected: u64 = 8589934592;
        let result = run_calc();
        assert_eq!(result, expected, "Test failed: expected {} but got {}", expected, result);
    }
}
