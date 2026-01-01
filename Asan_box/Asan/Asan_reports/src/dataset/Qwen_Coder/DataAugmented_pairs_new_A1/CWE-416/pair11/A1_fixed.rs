use std::ptr;
struct Resource {
    ptr: *mut i32,
}
impl Resource {
    unsafe fn obtain(&self) -> i32 {
        *self.ptr
    }
}
fn allocate() -> Resource {
    let boxed = Box::new(42);
    let raw = Box::into_raw(boxed);
    Resource { ptr: raw }
}
fn execute() {
    let resource = allocate();
    unsafe {
        let val = resource.obtain();
        println!("Value: {}", val);
    }
}
fn main() {
    execute();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Oracle test that validates the memory access.
    // For the vulnerable version, this test is expected to fail (undefined behavior),
    // while for the corrected version it should pass reliably.
    #[test]
    fn test_memory_access() {
        let resource = allocate();
        // For the vulnerable version, using the pointer after it has been freed may cause a crash or incorrect value.
        // For the corrected version, the returned value must equal 42.
        let value = unsafe { 
            // In the vulnerable version, this unsafe block causes use-after-free.
            // In the corrected version, `allocate` returns a safe Box.
            #[allow(unused_unsafe)]
            resource.obtain() 
        };
        assert_eq!(value, 42, "The obtained value must be 42");
    }
}
