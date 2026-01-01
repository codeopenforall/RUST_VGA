use std::sync::{Arc, Mutex};

pub struct Counter {
    value: u32,
}

impl Counter {
    pub fn new(initial_value: u32) -> Self {
        Counter {
            value: initial_value,
        }
    }

    pub fn subtract(&mut self, amount: u32) {
        self.value = self.value.wrapping_sub(amount);
    }

    pub fn get(&self) -> u32 {
        self.value
    }
}

fn run(counter: Arc<Mutex<Counter>>, amount: u32, times: usize) {
    for _ in 0..times {
        let mut c = counter.lock().unwrap();
        c.subtract(amount);
    }
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
