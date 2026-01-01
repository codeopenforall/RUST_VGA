use std::env;

struct Engine {
    counter: usize,
}

impl Engine {
    fn new() -> Self {
        Engine { counter: 0 }
    }

    fn update(&mut self, inc: usize) -> usize {
        unsafe {
            let ptr: *mut usize = &mut self.counter;
            *ptr = self.counter.wrapping_add(inc);
        }
        // Introducing a defect: changing the condition to allow counter to reach 100
        assert!(self.counter <= 100, "Counter exceeded safe threshold");
        self.counter
    }
}

fn run_app(input: String) -> usize {
    let value: usize = input.parse().unwrap();
    let mut eng = Engine::new();
    eng.update(value)
}

fn main() {
    let arg = env::args().nth(1).unwrap_or("0".to_string());
    let result = run_app(arg);
    println!("Final counter value: {}", result);
}

#[cfg(test)]
mod tests {
    use std::panic;

    // This test uses an input that would normally trigger the assertion in the insecure version.
    // When run against the insecure implementation, the test will panic (and thus fail),
    // while with the secure implementation the check prevents the panic.
    #[test]
    fn test_run_app_threshold() {
        // Input designed to cause the counter to reach or exceed the threshold.
        let input = "100".to_string();

        // Capturing the run_app call inside a catch_unwind block.
        let result = panic::catch_unwind(|| {
            // Assumes a function `run_app` exists in the current crate.
            crate::run_app(input)
        });

        // For the secure version, result is Ok and the counter remains below threshold.
        // For the insecure version, the assertion fires and result is an error.
        assert!(result.is_ok(), "Application panicked unexpectedly");
        let counter = result.unwrap();
        assert!(counter < 100, "Counter exceeded safe threshold");
    }
}
