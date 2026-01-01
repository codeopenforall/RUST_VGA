use std::sync::{Arc, Mutex};

const THRESHOLD: usize = 1000;

struct Data {
    value: usize,
}

impl Data {
    unsafe fn init(value: usize) -> Self {
        Data { value }
    }

    fn operate(&self) {
        assert!(self.value < THRESHOLD, "Value should be less than threshold");
    }
}

fn execute_app(input: usize) {
    let safe_input = if input < THRESHOLD { input } else { THRESHOLD - 1 };
    let shared = Arc::new(Mutex::new(unsafe { Data::init(safe_input) }));

    {
        let data = shared.lock().unwrap();
        data.operate();
    }

    {
        let data_main = shared.lock().unwrap();
        data_main.operate();
    }
}

#[cfg(test)]
mod tests {
    use std::panic;
    use super::execute_app;

    #[test]
    fn test_edge_input() {
        // Test with an input value equal to the threshold.
        // In the vulnerable version, this input would trigger the assertion and panic (leading to DoS).
        // In the fixed version, the input is sanitized so that no panic occurs.
        let result = panic::catch_unwind(|| {
            execute_app(1000);
        });
        // The oracle expects the application NOT to panic.
        assert!(
            result.is_ok(),
            "Application panicked with edge input value, indicating a vulnerability"
        );
    }
}
