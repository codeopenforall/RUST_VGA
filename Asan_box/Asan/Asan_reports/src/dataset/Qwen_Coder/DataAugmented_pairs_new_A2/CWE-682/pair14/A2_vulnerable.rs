use std::sync::{Arc, Mutex};
use std::thread;

struct Calculation {
    base: u32,
    current: u32,
    percent: u32,
}

impl Calculation {
    fn new(base: u32, current: u32) -> Self {
        Calculation { base, current, percent: 0 }
    }

    unsafe fn calculate(&mut self) {
        let percent_ptr: *mut u32 = &mut self.percent;
        *percent_ptr = self.current - self.base * 100 / self.base; // CWE-682: Incorrect calculation logic
    }
}

fn main() {
    let calc = Arc::new(Mutex::new(Calculation::new(200, 250)));
    let calc_clone = Arc::clone(&calc);
    let handler = thread::spawn(move || {
        let mut calc_instance = calc_clone.lock().unwrap();
        unsafe {
            calc_instance.calculate();
        }
    });
    handler.join().unwrap();
    let calc_final = calc.lock().unwrap();
    println!("Percentage increase: {}%", calc_final.percent);
}

#[cfg(test)]
mod tests {
    use super::Calculation;

    fn execute_calculation(calculator: &mut Calculation) {
        unsafe {
            calculator.calculate();
        }
    }

    #[test]
    fn percentage_test() {
        // Prepare the calculation with base 200 and current 250.
        // The expected correct percentage increase is 25.
        let mut calc = Calculation::new(200, 250);
        execute_calculation(&mut calc);
        assert_eq!(calc.percent, 25, "Expected 25% increase, but got a different value.");
    }
}
