extern crate libc;
use std::ptr;
use std::sync::Arc;
use std::thread;

struct Allocator {
    quantity: usize,
    element_size: usize,
}

impl Allocator {
    fn new(quantity: usize, elem_size: usize) -> Allocator {
        let total = unsafe {
            (quantity as u32).wrapping_mul(elem_size as u32) as usize
        };
        Allocator { quantity, element_size: total }
    }

    fn allocate(&self) -> *mut u8 {
        unsafe {
            let memory = libc::malloc(self.element_size) as *mut u8;
            if memory.is_null() {
                panic!("Memory allocation failed");
            }
            memory
        }
    }

    fn populate_buffer(&self, buffer: *mut u8) {
        let shared_buffer = Arc::new(buffer);
        let mut threads = vec![];
        for i in 0..4 {
            let arc_buffer = Arc::clone(&shared_buffer);
            threads.push(thread::spawn(move || {
                unsafe {
                    for j in 0..1000 {
                        let offset = i * 1000 + j;
                        ptr::write(arc_buffer.add(offset), 0xAAu8);
                    }
                }
            }));
        }
        for t in threads {
            t.join().unwrap();
        }
    }
}

fn main() {
    let alloc = Allocator::new(2_000_000_000, 4);
    let buffer = alloc.allocate();
    alloc.populate_buffer(buffer);
    println!("[Vulnerable] Calculated allocation size: {}", alloc.element_size);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // Test oracle function: the expected correct allocation size is 8_000_000_000.
    // In the vulnerable version, the computed size will wrap around to 3705032704.
    // In the fixed version, the multiplication is done safely.
    fn expected_size() -> usize {
        // 2_000_000_000 * 4 computed in 64-bit arithmetic without overflow.
        8_000_000_000
    }

    #[test]
    fn test_allocation_size() {
        // Test with the provided input values.
        let res = panic::catch_unwind(|| {
            let mgr = Manager::new(2_000_000_000, 4);
            mgr.size
        });
        
        match res {
            // For the vulnerable variant, the size is computed unsafely.
            // It will not panic but produce a truncated value.
            Ok(size) => {
                // Vulnerable version produces a size different from the expected value.
                assert_eq!(size, expected_size(), "Allocation size mismatch.");
            },
            // In the fixed code, if an integer overflow is detected, it would panic.
            Err(_) => {
                // If the fixed version panicked, then the vulnerability is mitigated by aborting.
                // However, we expect the fixed version NOT to panic with these inputs.
                panic!("Unexpected panic encountered in fixed version.");
            }
        }
    }
}

#[cfg(test)]
fn main_for_test() {
    tests::test_allocation_size();
}
