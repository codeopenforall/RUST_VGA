pub struct Memory {
    pub ptr: *mut i32,
}

impl Memory {
    pub fn new(value: i32) -> Self {
        let boxed = Box::new(value);
        Memory {
            ptr: Box::into_raw(boxed),
        }
    }

    pub unsafe fn get(&self) -> i32 {
        *self.ptr
    }
}

pub fn run() -> i32 {
    let mem = Memory::new(256);
    let raw = mem.ptr;
    unsafe {
        *raw = 9999;
        *raw
    }
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
