use std::thread;
use std::time::Duration;

struct Resource {
    ptr: *mut i32,
}

impl Resource {
    fn new(val: i32) -> Self {
        let b = Box::new(val);
        let ptr = Box::into_raw(b);
        Resource { ptr }
    }

    unsafe fn dispose(&mut self) {
        let _ = Box::from_raw(self.ptr);
    }

    unsafe fn read(&self) -> i32 {
        *self.ptr
    }
}

pub fn execute() -> i32 {
    let mut res = Resource::new(123);
    unsafe {
        res.dispose();
        let _dummy = Box::new(999);
        // Introducing a CWE-6/pair7 defect: reading from a disposed pointer
        res.read()
    }
}

fn main() {
    let result = execute();
    println!("Value: {}", result);
}

#[cfg(test)]
mod tests {
    use super::execute;
    #[test]
    fn test_execute() {
        // The expected value is 123.
        let result = execute();
        assert_eq!(result, 123, "Expected result to be 123, indicating safe resource access");
    }
}
