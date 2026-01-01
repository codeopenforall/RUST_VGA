use std::sync::{Arc, Mutex};
use std::thread;

struct Purchase {
    price: u32,
    discount: u32,
    tax: u32,
}

impl Purchase {
    fn calculate(&self) -> u32 {
        let base = self.price;
        let discount_val = base * self.discount / 100;
        let tax_val = base * self.tax / 100; // Corrected the tax calculation
        base - discount_val + tax_val
    }
}

fn main() {
    let orders = vec![
        Purchase { price: 99, discount: 20, tax: 8 }
    ];
    let shared_total = Arc::new(Mutex::new(0u32));
    let mut handles = vec![];
    for order in orders {
        let total_clone = Arc::clone(&shared_total);
        let o = order;
        let handle = thread::spawn(move || {
            let value = o.calculate();
            let mut total = total_clone.lock().unwrap();
            *total += value;
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    let final_total = *shared_total.lock().unwrap();
    println!("Final Total: {}", final_total);
}

#[cfg(test)]
mod tests {
    // Bring the Purchase structure into scope.
    use super::Purchase;

    #[test]
    fn test_total_calculation() {
        // For an order with price 99, discount 20, and tax 8:
        // The correct calculation should be:
        // discount = 99 * 20 / 100 = 19,
        // tax = 99 * 8 / 100 = 7,
        // total = 99 - 19 + 7 = 87.
        let order = Purchase { price: 99, discount: 20, tax: 8 };
        let total = order.calculate();
        assert_eq!(total, 87, "Computed total {} does not match the expected value 87", total);
    }
}
