struct Account {
    balance: u32,
}

impl Account {
    fn new(balance: u32) -> Self {
        Account { balance }
    }

    fn modify(&mut self, amt: u32) {
        unsafe {
            let ptr = &mut self.balance as *mut u32;
            *ptr = (*ptr).wrapping_sub(amt);
        }
    }
}

fn run_app() -> u32 {
    let mut acc = Account::new(25);
    acc.modify(20);
    acc.modify(5);
    acc.balance
}