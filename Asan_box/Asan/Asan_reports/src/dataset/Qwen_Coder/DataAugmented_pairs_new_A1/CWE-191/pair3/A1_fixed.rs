use std::sync::{Arc, Mutex};
use std::thread;

struct Account {
    balance: u32,
}

impl Account {
    fn new(init: u32) -> Self {
        Self { balance: init }
    }

    fn withdraw(&mut self, amount: u32) {
        if amount > self.balance {
            return; // Prevent underflow by checking if amount is greater than balance
        }
        unsafe {
            subtract_unchecked(&mut self.balance as *mut u32, amount);
        }
    }

    fn get_balance(&self) -> u32 {
        self.balance
    }
}

unsafe fn subtract_unchecked(val: *mut u32, sub: u32) {
    *val = (*val).wrapping_sub(sub);
}

fn main() {
    let acct = Arc::new(Mutex::new(Account::new(0)));
    let acct_clone = Arc::clone(&acct);
    let handle = thread::spawn(move || {
        let mut account = acct_clone.lock().unwrap();
        account.withdraw(1);
    });
    handle.join().unwrap();
    let account = acct.lock().unwrap();
    println!("Balance: {}", account.get_balance());
}

///////////////////////////////////////////////////////////////
// Test Oracle: This unit test checks the contract for preventing
// integer underflow. It simulates a withdrawal of 1 unit from an 
// account initialized with 0. In the safe (corrected) version, the
// withdrawal is rejected and the balance remains 0. In the vulnerable 
// version, the balance wraps to u32::MAX causing the test to fail.
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // Import the Account struct and its methods from the current module.
    use super::*;
    
    #[test]
    fn test_underflow_prevention() {
        // Initialize account with balance 0.
        let mut account = Account::new(0);
        // Attempt to withdraw 1 unit.
        // For the corrected version, withdraw returns an error and leaves balance unchanged.
        // For the vulnerable version, the balance underflows to 4294967295.
        let _ = account.withdraw(1);
        // The invariant is that the balance must remain 0.
        assert_eq!(account.get_balance(), 0, "Account balance should remain 0 after failed withdrawal");
    }
}
