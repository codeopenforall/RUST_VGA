use std::sync::atomic::{AtomicBool, Ordering};

struct Context {
    cancel_flag: AtomicBool,
    timeout_flag: AtomicBool,
}

impl Context {
    fn new(cancel: bool, timeout: bool) -> Self {
        Context {
            cancel_flag: AtomicBool::new(cancel),
            timeout_flag: AtomicBool::new(timeout),
        }
    }

    pub fn execute(&self) -> Result<i32, &'static str> {
        if self.timeout_flag.load(Ordering::SeqCst) {
            return Err("Operation timed out");
        }
        if self.cancel_flag.load(Ordering::SeqCst) {
            return Err("Operation cancelled");
        }
        unsafe {
            let raw_ptr = Box::into_raw(Box::new(256));
            let result = *raw_ptr;
            Box::from_raw(raw_ptr);
            Ok(result)
        }
    }
}