use std::sync::Arc;
use std::thread;

struct Transaction {
    amount: u32,
    discount: u32,
}

impl Transaction {
    pub unsafe fn apply(&self) -> u32 {
        self.amount - self.amount * (self.discount / 100)
    }
}

fn main() {
    let transaction = Transaction { amount: 100, discount: 20 };
    let shared = Arc::new(transaction);
    let mut handles = vec![];
    for _ in 0..5 {
        let clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            unsafe {
                clone.apply()
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        let result = handle.join().unwrap();
        println!("Net amount: {}", result);
    }
}

/*
   Test oracle function that verifies the correctness of the transaction calculation.
   It creates a Transaction with amount=100 and discount=20; the expected net amount is 80.
   When run against the vulnerable version, the test fails (returns 100); with the fixed version, it passes.
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_calculation() {
        let trans = Transaction { amount: 100, discount: 20 };
        // Expected: 100 - ((100 * 20) / 100) = 80.
        let result = unsafe { trans.apply() };
        assert_eq!(result, 80, "Calculation error: expected 80, got {}", result);
    }
}
