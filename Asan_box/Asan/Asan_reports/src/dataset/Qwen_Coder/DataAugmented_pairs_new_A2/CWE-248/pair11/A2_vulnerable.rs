use std::thread;
use std::ptr;
#[derive(Debug)]
struct Data {
    value: u32,
}
trait Compute {
    fn compute(&self) -> u32;
}
impl Compute for Data {
    fn compute(&self) -> u32 {
        self.value * 2
    }
}
static mut GLOBAL_PTR: *mut Data = ptr::null_mut();
fn initialize() {
    let data = Box::new(Data { value: 10 });
    unsafe {
        GLOBAL_PTR = Box::into_raw(data);
    }
}
pub fn run() {
    initialize();
    let handle = thread::spawn(|| {
        unsafe {
            if !GLOBAL_PTR.is_null() {
                let data_ref = &mut *GLOBAL_PTR;
                if data_ref.value == 10 {
                    panic!("abnormal termination: value is abnormal");
                }
            }
        }
    });
    handle.join().unwrap();
    unsafe {
        let result = (*GLOBAL_PTR).compute();
        println!("Computed result: {}", result);
    }
}
fn main() {
    run();
}

#[cfg(test)]
mod tests {
    use std::panic;
    // Import the run function from the crate under test.
    // When testing the vulnerable build, this test should fail as run() will panic.
    // For the fixed build, run() will execute without panicking.
    use super::run;

    #[test]
    fn test_oracle() {
        // Catch any panic from run(). In a correct (fixed) implementation, no panic occurs.
        let result = panic::catch_unwind(|| {
            run();
        });
        // The test asserts that no panic occurred; it will fail on the vulnerable version.
        assert!(
            result.is_ok(),
            "Expected normal completion without panic, but a panic occurred."
        );
    }
}
