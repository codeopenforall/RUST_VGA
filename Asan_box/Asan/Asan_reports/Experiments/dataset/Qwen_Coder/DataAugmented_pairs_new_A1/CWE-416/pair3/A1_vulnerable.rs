use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;
use std::thread;

struct Data {
    value: u32,
}

struct Container {
    inner: *mut Data,
}

impl Container {
    fn create(value: u32) -> Self {
        let boxed = Box::new(Data { value });
        Self { inner: Box::into_raw(boxed) }
    }

    unsafe fn access(&self) -> u32 {
        (*self.inner).value
    }
}

impl Drop for Container {
    fn drop(&mut self) {
        unsafe {
            if !self.inner.is_null() {
                Box::from_raw(self.inner);
            }
        }
    }
}

static GLOBAL_DATA: AtomicPtr<Data> = AtomicPtr::new(ptr::null_mut());

pub fn run_op() -> u32 {
    let container = Container::create(100);
    GLOBAL_DATA.store(container.inner, Ordering::SeqCst);
    drop(container);
    let handle = thread::spawn(|| unsafe {
        let p = GLOBAL_DATA.load(Ordering::SeqCst);
        (*p).value
    });
    let outcome = handle.join().unwrap();
    outcome
}

fn main() {
    let output = run_op();
    println!("Output: {}", output);
}

#[cfg(test)]
mod tests {
    // Import items from the current module.
    use super::run_op;

    #[test]
    fn test_oracle() {
        // The operation is expected to return 100.
        // In the vulnerable code this test is likely to fail or behave unpredictably due 
        // to use-after-free, whereas the corrected version will pass.
        let result = run_op();
        assert_eq!(result, 100, "Expected the result to be 100");
    }
}
