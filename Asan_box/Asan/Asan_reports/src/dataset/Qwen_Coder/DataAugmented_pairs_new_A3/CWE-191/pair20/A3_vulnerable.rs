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

#[cfg(test)]
mod tests {
    use super::*;
    
    // The test oracle verifies the differences between the two implementations.
    // For the vulnerable version, the unchecked subtraction causes an underflow leading to an incorrect final balance.
    // For the corrected version, the underflow is caught; thus only the valid subtraction (of 5) updates the balance.
    // This test asserts that the final balance is exactly 5.
    #[test]
    fn test_oracle() {
        let final_balance = run_app();
        assert_eq!(final_balance, 5, "Final balance should remain 5 after valid operations");
    }
}
