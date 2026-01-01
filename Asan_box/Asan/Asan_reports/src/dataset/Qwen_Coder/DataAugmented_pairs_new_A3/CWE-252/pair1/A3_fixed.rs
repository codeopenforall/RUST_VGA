use std::io;

// Global variable to simulate shared state
static mut GLOBAL: u32 = 0;

// Simulated unsafe update function that always fails
unsafe fn unsafe_update(value: u32) -> io::Result<()> {
    Err(io::Error::new(io::ErrorKind::Other, "Simulated update failure"))
}

// Public function to execute the update logic
pub fn execute() -> u32 {
    let value = 1500;
    match unsafe_update(value) {
        Ok(()) => {
            unsafe {
                GLOBAL = GLOBAL.wrapping_add(value);
            }
        }
        Err(e) => {
            println!("Update failed: {}", e);
        }
    }
    unsafe { GLOBAL }
}

#[cfg(test)]
mod tests {
    use std::sync::Once;
    
    static INIT: Once = Once::new();

    #[test]
    fn test_oracle() {
        unsafe {
            super::GLOBAL = 0;
        }
        let result = super::execute();
        assert_eq!(result, 0, "Test oracle: expected global state to remain 0 on update failure");
    }
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
