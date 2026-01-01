use std::ptr;

struct Memory {
    ptr: *mut i32,
}

impl Memory {
    fn new(val: i32) -> Self {
        let boxed_val = Box::new(val);
        let raw_ptr = Box::into_raw(boxed_val);
        Memory { ptr: raw_ptr }
    }

    unsafe fn get(&self) -> i32 {
        *self.ptr
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.ptr);
        }
    }
}

fn run() -> i32 {
    let mem = Memory::new(256);
    let raw_ptr = mem.ptr;
    drop(mem);
    unsafe {
        *raw_ptr = 9999; // Defect: Writing to a freed memory location
        *raw_ptr
    }
}

fn main() {
    println!("Result: {}", run());
}

#[cfg(test)]
mod tests {
    // The test oracle asserts that the function returns 256.
    // With the vulnerable version, the use‐after‐free corrupts the value (returning 9999) and the test will fail.
    // With the fixed version, the test passes.
    #[test]
    fn memory_safety_test() {
        let result = crate::run();
        assert_eq!(result, 256, "Test failed: memory value corrupted due to use‐after‐free vulnerability");
    }
}
