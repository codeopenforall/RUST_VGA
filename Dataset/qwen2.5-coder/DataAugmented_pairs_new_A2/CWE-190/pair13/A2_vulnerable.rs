use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::sync::{Arc, Mutex};
use std::thread;

struct Processor {
    quantity: u32,
    item_size: u32,
}

impl Processor {
    fn new(quantity: u32, item_size: u32) -> Self {
        Processor { quantity, item_size }
    }

    fn calculate_size(&self) -> usize {
        (self.quantity.wrapping_mul(self.item_size)) as usize
    }

    fn execute(&self) {
        let size = self.calculate_size();
        let layout = unsafe { Layout::from_size_align_unchecked(size, 8) };
        unsafe {
            let ptr = alloc_zeroed(layout);
            for i in 0..size {
                *ptr.add(i) = 1;
            }
            dealloc(ptr, layout);
        }
        println!("Execution complete. Calculated size: {}", size);
    }
}

fn main() {
    let processor = Arc::new(Processor::new(2_000_000_000, 3));
    let status = Arc::new(Mutex::new(false));
    let mut threads = Vec::new();
    for _ in 0..4 {
        let p = Arc::clone(&processor);
        let s = Arc::clone(&status);
        let t = thread::spawn(move || {
            p.execute();
            let mut completed = s.lock().unwrap();
            *completed = true;
        });
        threads.push(t);
    }
    for t in threads {
        t.join().unwrap();
    }
    assert!(*status.lock().unwrap());
    println!("Main completed execution.");
}