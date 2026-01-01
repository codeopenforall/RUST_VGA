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