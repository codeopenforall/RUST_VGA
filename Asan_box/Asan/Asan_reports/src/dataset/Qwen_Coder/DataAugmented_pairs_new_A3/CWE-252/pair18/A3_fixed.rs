use std::sync::{Arc, Mutex};
use std::thread;

struct SecureData {
    value: i32,
}

impl SecureData {
    unsafe fn update(&mut self, delta: i32) -> Result<(), &'static str> {
        if self.value + delta < 0 || self.value + delta > 100 {
            return Err("value out of range");
        }
        self.value += delta;
        Ok(())
    }
}

fn process(data: Arc<Mutex<SecureData>>, delta: i32) {
    let mut handles = Vec::new();
    for _ in 0..5 {
        let data_cloned = Arc::clone(&data);
        let h = thread::spawn(move || {
            unsafe {
                let mut guard = data_cloned.lock().unwrap();
                let _ = guard.update(delta);
            }
        });
        handles.push(h);
    }
    for h in handles {
        h.join().unwrap();
    }
}

pub fn run() -> i32 {
    let initial = SecureData { value: 50 };
    let shared = Arc::new(Mutex::new(initial));
    process(Arc::clone(&shared), 10);
    process(Arc::clone(&shared), 10); // Change 60 to 10 to prevent overflow
    let final_val = shared.lock().unwrap().value;
    final_val
}

fn main() {
    let final_value = run();
    println!("Final value: {}", final_value);
}

/*
The following test oracle function is used to verify the behavior of the two versions.
It calls the exposed run() function and asserts that the returned final value is safe (i.e., it equals 100).
For the vulnerable version, the final value will be 400, causing the test to fail.
For the fixed version, the final value remains 100, and the test passes.
*/
#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn verify_final_state() {
        let final_value = run();
        // In a secure state, the final value should remain 100.
        assert_eq!(final_value, 100, "Final value is unsafe due to unchecked error handling");
    }
}
