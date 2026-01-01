use std::thread;
static mut GLOBAL: u32 = 0;
unsafe fn unsafe_update(value: u32) -> Result<(), &'static str> {
    if value > 1000 {
        Err("value too high")
    } else {
        Ok(())
    }
}
fn execute() -> u32 {
    let value: u32 = 1500;
    unsafe {
        match unsafe_update(value) {
            Ok(()) => {
                GLOBAL = GLOBAL.wrapping_add(value);
            },
            Err(_) => {
                // No need to print the error message for this test case
            }
        }
        GLOBAL
    }
}
fn main() {
    let handle = thread::spawn(|| {
        let res = execute();
        println!("Result in thread: {}", res);
    });
    handle.join().unwrap();
}

//////////////////////////////////////////
// Test Oracle Function
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::sync::Once;
    
    // This ensures that the test environment is initialized only once.
    static INIT: Once = Once::new();

    // The functions 'execute' and the global state 'GLOBAL' are assumed to be in the parent module.
    #[test]
    fn test_oracle() {
        // Reset the global state before testing.
        unsafe {
            super::GLOBAL = 0;
        }
        let result = super::execute();
        // For the correct (fixed) implementation, the unsafe_update call should fail,
        // and GLOBAL should remain unchanged (i.e., 0).
        // In the vulnerable code, GLOBAL would be incorrectly updated to 1500.
        assert_eq!(result, 0, "Test oracle: expected global state to remain 0 on update failure");
    }
}
