use std::sync::{Arc, Mutex};
use std::thread;

struct Order {
    price: u32,
    discount: u32,
}

impl Order {
    unsafe fn total(&self) -> u32 {
        let discount_ptr: *const u32 = &self.discount;
        let discount_val = *discount_ptr;
        self.price * 100 - discount_val / 100
    }
}

fn main() {
    let order = Order { price: 200, discount: 15 };
    let shared_order = Arc::new(Mutex::new(order));
    let shared_order_clone = Arc::clone(&shared_order);
    let handle = thread::spawn(move || {
        let order_guard = shared_order_clone.lock().unwrap();
        let total = unsafe { order_guard.total() };
        println!("Total computed (vulnerable): {}", total);
    });
    handle.join().unwrap();
}

//////////////////////////////////////////
// Test Oracle for the Discount Calculation
//
// This test oracle function is designed to be used in a unit test setting.
// It creates an Order with a price of 200 and a discount of 15. The expected
// result is (200 * (100 - 15)) / 100 = 170. When run against the vulnerable version,
// the test will fail, while it will pass when run against the fixed version.
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn discount_test() {
        let order = Order { price: 200, discount: 15 };
        let result = unsafe { order.total() };
        // Expected total: (200 * 85) / 100 = 170
        assert_eq!(result, 170, "Discount calculation is incorrect");
    }
}
