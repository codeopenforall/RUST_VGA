struct Holder {
    ptr: *mut i32,
}

impl Holder {
    fn new(value: i32) -> Self {
        Holder {
            ptr: Box::into_raw(Box::new(value)),
        }
    }

    unsafe fn deallocate(&mut self) {
        let _ = Box::from_raw(self.ptr as *mut i32);
    }

    unsafe fn read(&self) -> i32 {
        *self.ptr
    }
}

fn process(holder: &Holder) -> i32 {
    unsafe { holder.read() }
}

//////////////////////////////////////////
// Test Oracle for Use-After-Free Issue //
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    // Import all definitions from the parent module.
    use super::*;

    #[test]
    fn test_dangling_access() {
        let mut resource = Holder::new(12345);
        unsafe {
            resource.deallocate();
        }
        let result = process(&resource);
        // In the fixed version, the read function returns -1 after deallocation.
        // In the vulnerable version, the behavior is undefined and almost never -1.
        assert_eq!(result, -1, "The resource was accessed after being freed!");
    }
}
