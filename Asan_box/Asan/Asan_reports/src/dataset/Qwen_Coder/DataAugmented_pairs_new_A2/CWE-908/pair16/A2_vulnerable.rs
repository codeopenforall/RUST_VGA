#![allow(dead_code)]
use std::mem::MaybeUninit;
#[repr(C)]
pub struct Config {
    pub name: String,
    pub count: u32,
}
impl Config {
    pub unsafe fn build_partial(count: u32) -> Self {
        let mut storage = MaybeUninit::<Config>::uninit();
        let ptr = storage.as_mut_ptr();
        let count_ptr = &mut (*ptr).count as *mut u32;
        count_ptr.write(count);
        storage.assume_init()
    }
    pub fn valid(&self) -> bool {
        !self.name.is_empty() && self.count > 0
    }
}
pub fn compute() -> bool {
    unsafe {
        let cfg = Config::build_partial(42);
        // Introducing a vulnerability by accessing an uninitialized field
        let _name_len = cfg.name.len(); // This will cause a panic because `name` is not initialized
        cfg.valid()
    }
}
fn main() {
    let result = std::panic::catch_unwind(|| {
        compute()
    });
    match result {
        Ok(validity) => println!("Configuration valid: {}", validity),
        Err(_) => println!("Runtime error occurred during configuration processing."),
    }
}

////////////////////////////////////////////////////////////
// Test Oracle Function
////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // The test expects that compute() runs successfully and returns true in the fixed version.
    // In the vulnerable version, compute() will invoke undefined behavior (likely panicking).
    use super::compute;

    #[test]
    fn oracle() {
        let result = std::panic::catch_unwind(|| {
            compute()
        });
        // For the vulnerable version this should be an error (i.e. is_err() is true),
        // while for the fixed version the execution should succeed and return true.
        if result.is_err() {
            panic!("Test failed: The configuration processing panicked (vulnerability present)");
        }
        let valid = result.unwrap();
        assert!(valid, "Test failed: The configuration was not valid (unexpected value)");
    }
}
