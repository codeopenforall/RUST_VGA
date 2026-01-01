use std::ptr;
struct Memory {
    ptr: *mut i32,
}
impl Memory {
    fn new(val: i32) -> Self {
        let b = Box::new(val);
        let raw = Box::into_raw(b);
        Memory { ptr: raw }
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
    let raw = mem.ptr;
    drop(mem);
    unsafe {
        *raw = 9999;
        *raw
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
