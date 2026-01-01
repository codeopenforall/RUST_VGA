use std::thread;

fn approved(x: i32) -> i32 {
    x * 2
}

fn diverted(x: i32) -> i32 {
    x + 45
}

static mut ACTION: fn(i32) -> i32 = approved;

pub fn run_app() -> i32 {
    let handler = thread::spawn(|| {
        unsafe {
            ACTION = diverted;
        }
    });

    handler.join().unwrap();

    let result = unsafe { ACTION(5) };
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
