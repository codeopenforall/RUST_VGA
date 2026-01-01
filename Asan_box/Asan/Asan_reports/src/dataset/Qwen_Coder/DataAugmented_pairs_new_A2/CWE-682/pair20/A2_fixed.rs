use std::sync::{Arc, Mutex};
use std::thread;

pub struct Calculator {
    pub scale: i32,
}

impl Calculator {
    pub unsafe fn compute(&self, numerator: i32, denominator: i32) -> i32 {
        let scale_val = self.scale;
        let result = numerator / (denominator * scale_val);
        result
    }
}

fn launch_calculation(calc: Arc<Calculator>, num: i32, denom: i32, store: Arc<Mutex<i32>>) {
    let calc_clone = calc.clone();
    thread::spawn(move || {
        unsafe {
            let computed = calc_clone.compute(num, denom);
            let mut data = store.lock().unwrap();
            *data = computed;
        }
    })
    .join()
    .unwrap();
}

fn main() {
    let calc = Arc::new(Calculator { scale: 10 });
    let result = Arc::new(Mutex::new(0));
    launch_calculation(calc, 100, 5, result.clone());
    let final_value = *result.lock().unwrap();
    println!("Computed result: {}", final_value);
}

//////////////////////////////////////////////////////////////
// The test oracle below exercises the compute method of the Calculator.
// When run against the flawed implementation, the result will be 200,
// causing the assertion to fail. With the corrected implementation,
// the result will be 2 and the test will pass.
//
// This test does not rely on printed output and uses explicit assertions.
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::Calculator;
    #[test]
    fn oracle_test() {
        let calc = Calculator { scale: 10 };
        // The chosen inputs: numerator=100 and denominator=5.
        // The intended correct calculation: 100/(5*10)=2.
        unsafe {
            let computed = calc.compute(100, 5);
            assert_eq!(computed, 2, "Computation error: expected 2, got {}", computed);
        }
    }
}
