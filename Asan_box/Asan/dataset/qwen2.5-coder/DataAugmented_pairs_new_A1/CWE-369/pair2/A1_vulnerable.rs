use std::sync::Arc;
use std::thread;

struct Calculation {
    top: i32,
    bottom: i32,
}

impl Calculation {
    fn perform(&self) -> i32 {
        unsafe {
            let ptr = &self.top as *const i32;
            let value = *ptr;
            value / self.bottom
        }
    }
}

fn main() {
    let calc = Arc::new(Calculation { top: 100, bottom: 0 });
    let calc_clone = Arc::clone(&calc);
    let handle = thread::spawn(move || {
        calc_clone.perform()
    });
    let outcome = handle.join().expect("Thread panicked due to divide-by-zero");
    println!("Outcome: {}", outcome);
}