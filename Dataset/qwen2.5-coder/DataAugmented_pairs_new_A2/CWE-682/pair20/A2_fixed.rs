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