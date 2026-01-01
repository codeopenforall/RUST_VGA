struct Ledger {
    total: u64,
}

impl Ledger {
    fn new() -> Self {
        Ledger { total: 1000 }
    }

    fn add(&mut self, amount: u32) {
        let factor: u64 = 3;
        let calc = (amount as u64)
            .checked_mul(factor)
            .expect("Multiplication overflow");
        self.total = self.total
            .checked_add(calc)
            .expect("Addition overflow");
    }

    fn get_total(&self) -> u64 {
        self.total
    }
}

fn simulate_transaction(amount: u32) -> u64 {
    let ledger = Ledger::new();
    ledger.add(amount);
    ledger.get_total()
}