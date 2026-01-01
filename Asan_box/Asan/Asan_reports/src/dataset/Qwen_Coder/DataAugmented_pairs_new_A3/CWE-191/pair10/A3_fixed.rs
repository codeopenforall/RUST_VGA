use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Ledger {
    value: AtomicUsize,
}

impl Ledger {
    pub fn new(initial_value: usize) -> Self {
        Ledger {
            value: AtomicUsize::new(initial_value),
        }
    }

    pub fn withdraw(&self, amount: usize) {
        let current = self.value.load(Ordering::SeqCst);
        let new = current.saturating_sub(amount);
        self.value.store(new, Ordering::SeqCst);
    }

    pub fn get(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }
}

///////////////////////////////
// Test Oracle Function
///////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    // This test will assert the final balance. The secure variant (fixed)
    // should result in 0 given the input withdrawals. The vulnerable variant
    // produces a large wrapped-around value, causing the test to fail.
    #[test]
    fn test_underflow_oracle() {
        let ledger = Arc::new(Ledger::new(100));
        let mut handles = Vec::new();

        {
            let ledger_clone = Arc::clone(&ledger);
            handles.push(thread::spawn(move || {
                ledger_clone.withdraw(150);
            }));
        }
        {
            let ledger_clone = Arc::clone(&ledger);
            handles.push(thread::spawn(move || {
                ledger_clone.withdraw(10);
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        // Expected final balance is 0 if underflow is prevented.
        assert_eq!(ledger.get(), 0, "Final ledger value should be clamped to 0 to prevent underflow");
    }
}
