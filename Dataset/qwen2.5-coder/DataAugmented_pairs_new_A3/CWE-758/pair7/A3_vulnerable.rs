struct Account {
    balance: u32,
}

impl Account {
    pub fn compute(&self, rate: u32) -> u32 {
        (self.balance + self.balance * rate) / 100
    }
}