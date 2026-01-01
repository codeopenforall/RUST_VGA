struct Transaction {
    amount: u32,
    discount: u32,
}

impl Transaction {
    pub unsafe fn apply(&self) -> u32 {
        self.amount - self.amount * (self.discount / 100)
    }
}