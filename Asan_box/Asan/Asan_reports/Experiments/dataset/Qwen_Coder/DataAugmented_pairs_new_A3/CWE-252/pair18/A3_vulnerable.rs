struct Guard;

impl Guard {
    fn update(&mut self, delta: i32) -> Result<(), String> {
        if delta < 0 {
            Err("Negative delta".to_string())
        } else {
            Ok(())
        }
    }
}

pub struct Counter {
    value: i32,
    guard: Guard,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            value: 0,
            guard: Guard,
        }
    }

    pub fn increment(&mut self, delta: i32) {
        self.value += delta;
        let _ = self.guard.update(delta); // Intentionally ignoring the Result
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

pub fn run() -> i32 {
    let mut counter = Counter::new();
    counter.increment(100);
    counter.increment(200);
    counter.increment(-100); // This should fail but is ignored
    counter.get_value()
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
