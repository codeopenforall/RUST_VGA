use std::sync::Arc;
use std::thread;

pub struct Calc {
    rate: u32,
}

impl Calc {
    pub fn compute(&self, revenue: u32) -> u32 {
        unsafe {
            let rate_ptr: *const u32 = &self.rate as *const u32;
            let r = *rate_ptr;
            revenue * (r / 100 + 1) // Introducing a defect by adding 1 to the rate division
        }
    }
}

fn main() {
    let calc = Arc::new(Calc { rate: 5 });
    let calc_clone = Arc::clone(&calc);
    let handle = thread::spawn(move || {
        let result = calc_clone.compute(200);
        println!("Thread computed result: {}", result);
        result
    });
    let thread_result = handle.join().unwrap();
    println!("Main thread received result: {}", thread_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    // The following test oracle verifies the correctness of the calculation.
    // For a revenue of 200 and a rate of 5, the correct deduction should be 10.
    // In the vulnerable version the result will be 0, causing the test to fail.
    #[test]
    fn test_calculation() {
        let calc = Calc { rate: 5 };
        let result = calc.compute(200);
        assert_eq!(result, 10, "Expected deduction of 10 for revenue 200 with a 5% rate");
    }
}
