#![allow(dead_code)]
use std::thread;
use std::sync::Arc;
use std::ptr;

struct Handler {
    threshold: u32,
}

impl Handler {
    fn new() -> Result<Self, &'static str> {
        let config_content = load_config("config.txt")?; 
        let threshold = config_content.trim().parse::<u32>().map_err(|_| "Invalid threshold format")?;
        if threshold == 0 {
            return Err("Threshold must be greater than zero");
        }
        Ok(Handler { threshold })
    }
}

unsafe fn get_global() -> &'static Handler {
    &*GLOBAL_HANDLE
}

static mut GLOBAL_HANDLE: *mut Handler = ptr::null_mut();

fn load_config(_path: &str) -> Result<String, &'static str> {
    Err("failed to load configuration")
}

fn main() {
    let instance = match Handler::new() {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("Configuration error encountered: {}", e);
            return;
        }
    };
    let boxed = Box::new(instance);
    unsafe {
        GLOBAL_HANDLE = Box::into_raw(boxed);
    }
    let handle_arc = Arc::new(());
    let thread_handle = {
        let arc_clone = Arc::clone(&handle_arc);
        thread::spawn(move || {
            let _ = arc_clone;
            unsafe {
                let conf = get_global();
                println!("Operating with secure threshold: {}", conf.threshold);
            }
        })
    };
    thread_handle.join().unwrap();
    unsafe {
        if !GLOBAL_HANDLE.is_null() {
            drop(Box::from_raw(GLOBAL_HANDLE));
            GLOBAL_HANDLE = ptr::null_mut();
        }
    }
}

///////////////////////////////////////////////////////////////
// Test Oracle Function
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod oracle_tests {
    // Import everything from the surrounding module.
    use super::*;

    // This test validates the configuration initialization behavior.
    // The contract is:
    //   - For the vulnerable version: Handler::new() returns a value with threshold == 0,
    //     which violates the expected invariant and causes the test to fail.
    //   - For the fixed version: Handler::new() returns an error (or does not produce an insecure default),
    //     so the test passes.
    #[test]
    fn test_configuration_handling() {
        // The test handles both possible types of return.
        // For the vulnerable version, new() returns Self (and thus Ok(config) by design),
        // but the threshold value will be 0.
        // For the fixed version, new() returns Result<Self, &str> and will yield an Err.
        // We use a match to support both behaviors.
        let result = std::panic::catch_unwind(|| {
            // Note: In the vulnerable version, this call returns a Handler directly.
            // In the fixed version, it returns a Result<Handler, &str>.
            Handler::new()
        });

        match result {
            Ok(val) => {
                // If the call succeeded and returned a Handler, we extract the threshold.
                // Support both possible types via pattern matching on the dynamic type:
                // For vulnerable code, val is a Handler.
                // For fixed code, val is a Result<Handler, &str>.
                // We try to downcast to Result first.
                // Since Rust’s static typing does not allow runtime type inspection,
                // we simulate the contract by checking if val (as if it were a Handler) has an insecure threshold.
                // SAFETY: We use a transmute trick for testing purposes only.
                // In the fixed version, we expect never to reach here.
                let insecure_threshold = unsafe { *( &val as *const _ as *const u32) };
                assert!(insecure_threshold > 0, "Insecure default threshold accepted – configuration error swallowed");
            },
            Err(_) => {
                // For the fixed version, an error should occur during initialization.
                // Test passes in this branch.
            }
        }
    }
}
