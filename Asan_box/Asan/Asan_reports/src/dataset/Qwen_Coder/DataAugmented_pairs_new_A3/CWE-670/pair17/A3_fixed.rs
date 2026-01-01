use std::sync::{Arc, Mutex};
use std::thread;

struct Regulator {
    op: Mutex<fn(i32) -> i32>,
}

impl Regulator {
    fn new() -> Self {
        Regulator { op: Mutex::new(approved) }
    }

    fn update(&self, new_op: fn(i32) -> i32, token: &str) {
        if token == "secret_token" {
            let mut op_guard = self.op.lock().unwrap();
            *op_guard = new_op;
        }
    }

    fn execute(&self, input: i32) -> i32 {
        let op_guard = self.op.lock().unwrap();
        op_guard(input)
    }
}

fn approved(x: i32) -> i32 {
    x * 2
}

fn diverted(x: i32) -> i32 {
    x + 45
}

pub fn run_app() -> i32 {
    let regulator = Arc::new(Regulator::new());
    let reg_clone = Arc::clone(&regulator);

    let handler = thread::spawn(move || {
        reg_clone.update(diverted, "wrong_token");
    });

    handler.join().unwrap();

    let result = regulator.execute(5);
    result
}

#[cfg(test)]
mod tests {
    use super::run_app;
    
    #[test]
    fn control_flow_integrity_test() {
        // For approved computation, input 5 should yield 10.
        // In the vulnerable version the unsafely updated branch would yield 50.
        // This test should fail on the vulnerable version and pass on the fixed version.
        let res = run_app();
        assert_eq!(res, 10, "Control flow hijacking detected: computed value is not as expected");
    }
}
