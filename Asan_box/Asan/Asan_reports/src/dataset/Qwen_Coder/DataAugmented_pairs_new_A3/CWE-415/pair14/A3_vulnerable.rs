use std::sync::atomic::{AtomicBool, Ordering};
static mut FREED: bool = false;

struct Data {
    value: u32,
}

unsafe fn free_data(ptr: *mut Data) {
    if FREED {
        panic!("Double free detected!");
    }
    Box::from_raw(ptr);
    FREED = true;
    // Uncommenting the following line would fix the vulnerability
    // FREED = false;
}

fn app_run() {
    let boxed = Box::new(Data { value: 42 });
    let raw = Box::into_raw(boxed);
    unsafe {
        free_data(raw);
        free_data(raw);
    }
}

fn main() {
    app_run();
}

//////////////////////////////////////////////
// Test Oracle: Validate Memory Deallocation Integrity
// The test calls app_run() and asserts no panic occurs.
// Under the vulnerable version, the double free would trigger a panic,
// while the corrected version runs to completion.
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;
    // Import app_run from the module under test.
    use crate::app_run;

    #[test]
    fn test_memory_release() {
        let result = panic::catch_unwind(|| {
            app_run();
        });
        // The absence of a panic indicates the vulnerability has been fixed.
        assert!(result.is_ok(), "app_run panicked due to double free vulnerability.");
    }
}
