use std::sync::{Arc, Mutex};
use std::thread;

struct Order {
    price: u32,
    discount: u32,
}

impl Order {
    fn total(&self) -> u32 {
        (self.price * (100 - self.discount)) / 100
    }
}

fn main() {
    let order = Order { price: 200, discount: 15 };
    let shared_order = Arc::new(Mutex::new(order));
    let shared_order_clone = Arc::clone(&shared_order);
    let handle = thread::spawn(move || {
        let order_guard = shared_order_clone.lock().unwrap();
        let total = order_guard.total();
        println!("Total computed (fixed): {}", total);
    });
    handle.join().unwrap();
}