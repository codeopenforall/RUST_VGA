pub trait Worker {
    fn evaluate(&self) -> u32;
}

pub struct Processor {
    ptr: *mut u32,
}

impl Processor {
    pub fn new(val: u32) -> Self {
        let b = Box::new(val);
        Self {
            ptr: Box::into_raw(b),
        }
    }

    pub unsafe fn compute(&self) -> u32 {
        *self.ptr
    }
}

impl Drop for Processor {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.ptr);
        }
    }
}

pub fn execute() -> u32 {
    let result: u32;
    let local_ptr: *mut u32;
    {
        let job = Processor::new(42);
        local_ptr = job.ptr;
    } 
    unsafe {
        result = *local_ptr; // Potential defect: accessing dangling pointer after job is dropped
    }
    result
}

fn main() {
    let res = execute();
    println!("Result is: {}", res);
}

#[cfg(test)]
mod tests {
    // Since the vulnerable and fixed binaries each expose an 'execute' function returning a u32,
    // this test can be used to validate the correct behavior (expected to be 42).
    use super::execute;

    #[test]
    fn oracle_test() {
        let result = execute();
        // The test asserts that the computed result is exactly 42.
        // In the vulnerable version, undefined behavior may cause a failure or an incorrect value,
        // while the fixed version correctly returns 42.
        assert_eq!(result, 42, "Expected result to be 42");
    }
}
