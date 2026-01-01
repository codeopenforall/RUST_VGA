use std::sync::{Arc, Mutex};
use std::thread;

struct Counter {
    value: u64,
}

impl Counter {
    fn new(init: u64) -> Self {
        Counter { value: init }
    }

    fn decrease(&mut self, amount: u64) {
        if self.value >= amount {
            self.value -= amount;
        } else {
            self.value = 0;
        }
    }

    fn get(&self) -> u64 {
        self.value
    }
}

fn run(counter: Arc<Mutex<Counter>>, amount: u64, iterations: u32) {
    let mut handles = vec![];
    for _ in 0..iterations {
        let c = counter.clone();
        handles.push(thread::spawn(move || {
            let mut lock = c.lock().unwrap();
            lock.decrease(amount);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    let counter = Arc::new(Mutex::new(Counter::new(50)));
    run(counter.clone(), 10, 10);
    let final_value = counter.lock().unwrap().get();
    println!("Final count: {}", final_value);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_saturation() {
        // Here we instantiate the counter with 50 and attempt to subtract 10 ten times.
        // The expected correct behavior is that the counter saturates at 0 instead of wrapping around.
        let counter = Arc::new(Mutex::new(Counter::new(50)));
        run(counter.clone(), 10, 10);
        let final_value = counter.lock().unwrap().get();
        // The test will fail for the vulnerable version (which underflows)
        // and pass for the fixed version (which saturates at 0).
        assert_eq!(final_value, 0, "Counter did not saturate to zero as expected");
    }
}
