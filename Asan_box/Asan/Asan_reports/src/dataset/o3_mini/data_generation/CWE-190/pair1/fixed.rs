use std::sync::{Arc, Mutex};
use std::thread;
struct Accumulator {
    value: u8,
}
impl Accumulator {
    fn new(init: u8) -> Self {
        Self { value: init }
    }
    fn update(&mut self, add: u8) -> Result<(), &'static str> {
        if let Some(new_val) = self.value.checked_add(add) {
            self.value = new_val;
            Ok(())
        } else {
            Err("integer overflow detected")
        }
    }
    fn get(&self) -> u8 {
        self.value
    }
}
fn run_calculation() -> u8 {
    let mut acc = Accumulator::new(250);
    let _ = acc.update(10).unwrap_or_else(|err| {
        eprintln!("Warning: {}", err);
    });
    acc.get()
}
fn main() {
    let shared = Arc::new(Mutex::new(run_calculation()));
    let mut threads = vec![];
    for _ in 0..2 {
        let s = Arc::clone(&shared);
        threads.push(thread::spawn(move || {
            let val = *s.lock().unwrap();
            println!("Thread sees value: {}", val);
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    println!("Final result: {}", run_calculation());
}


#[allow(dead_code)]
fn test_oracle() {
    // The fixed version of run_calculation should prevent overflow and leave the value unchanged at 250.
    let result = run_calculation();
    // In the vulnerable version, the result would be 4 (due to wrap-around), so this test should fail.
    assert_eq!(result, 250, "The accumulator value is incorrect; expected 250 when preventing overflow");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_overflow_prevention() {
        test_oracle();
    }
}
