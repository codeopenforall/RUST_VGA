struct SafeData {
    ptr: Option<*mut i32>,
}

impl SafeData {
    fn new(value: i32) -> Self {
        let boxed = Box::new(value);
        SafeData { ptr: Some(Box::into_raw(boxed)) }
    }

    fn get(&self) -> i32 {
        unsafe { *self.ptr.expect("Pointer should be valid") }
    }
}

impl Drop for SafeData {
    fn drop(&mut self) {
        if let Some(p) = self.ptr.take() {
            unsafe {
                let _ = Box::from_raw(p);
            }
        }
    }
}

fn run_app() {
    let instance = SafeData::new(100);
    instance.get();
}

#[cfg(test)]
mod tests {
    use std::panic;

    // The test oracle ensures that the run_app function does NOT panic.
    // In the presence of the double free vulnerability, the vulnerable version will panic,
    // causing the test to fail. The corrected version, however, will run to completion.
    //
    // Note: The test is identical for both versions and will report a failure if a panic is caught.
    #[test]
    fn test_run_app_integrity() {
        let result = panic::catch_unwind(|| {
            // Depending on which version is compiled (vulnerable or corrected),
            // this call will either panic (vulnerable) or succeed (corrected).
            crate::run_app();
        });
        assert!(result.is_ok(), "Application panicked due to double free vulnerability");
    }
}
