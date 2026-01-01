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