use std::alloc::{Layout, alloc};
use std::ptr::null_mut;

struct Handler {
    count: u32,
    element_size: u32,
}

impl Handler {
    pub fn new(count: u32, element_size: u32) -> Self {
        Handler { count, element_size }
    }

    pub fn compute_size(&self) -> usize {
        let product = self.count.checked_mul(self.element_size)
            .expect("Integer multiplication overflow detected");
        product as usize
    }

    pub fn allocate(&self) -> *mut u8 {
        let size = self.compute_size();
        let layout = Layout::from_size_align(size, 8).expect("Invalid layout");
        unsafe { alloc(layout) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn safe_product(count: u32, element_size: u32) -> usize {
        let prod = (count as u64) * (element_size as u64);
        prod as usize
    }

    #[test]
    fn test_allocation_size() {
        let count = 2_000_000_000;
        let element_size = 3;
        let handler = Handler::new(count, element_size);
        let computed = handler.compute_size();
        let expected = safe_product(count, element_size);
        assert_eq!(computed, expected, "Computed size does not match expected safe multiplication result");
    }
}