use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

struct Ledger {
    value: UnsafeCell<u32>,
}

unsafe impl Sync for Ledger {}

impl Ledger {
    fn new(initial: u32) -> Self {
        Ledger {
            value: UnsafeCell::new(initial),
        }
    }

    fn withdraw(&self, amount: u32) {
        unsafe {
            let current = *self.value.get();
            *self.value.get() = current.saturating_sub(amount);
        }
    }

    fn get(&self) -> u32 {
        unsafe { *self.value.get() }
    }
}

fn main() {
    let ledger = Arc::new(Ledger::new(100));
    let mut threads = Vec::new();
    {
        let ledger_clone = Arc::clone(&ledger);
        threads.push(thread::spawn(move || {
            ledger_clone.withdraw(150);
        }));
    }
    {
        let ledger_clone = Arc::clone(&ledger);
        threads.push(thread::spawn(move || {
            ledger_clone.withdraw(10);
        }));
    }
    for thr in threads {
        thr.join().unwrap();
    }
    println!("Final value: {}", ledger.get());
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
