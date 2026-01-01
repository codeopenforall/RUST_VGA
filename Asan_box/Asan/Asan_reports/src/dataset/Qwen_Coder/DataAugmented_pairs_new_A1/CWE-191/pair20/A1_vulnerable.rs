use std::sync::{Arc, Mutex, Barrier};
use std::thread;

struct Account {
    balance: u32,
}

impl Account {
    fn new(b: u32) -> Self {
        Self { balance: b }
    }

    fn modify(&mut self, amt: u32) {
        unsafe {
            let ptr = &mut self.balance as *mut u32;
            *ptr = (*ptr).wrapping_sub(amt);
        }
    }

    fn retrieve(&self) -> u32 {
        self.balance
    }
}

fn run_app() -> u32 {
    let account = Arc::new(Mutex::new(Account::new(10)));
    let barrier_start = Arc::new(Barrier::new(2));
    let barrier_sync = Arc::new(Barrier::new(2));
    let acc_clone = account.clone();
    let barrier_start_clone = barrier_start.clone();
    let barrier_sync_clone = barrier_sync.clone();

    let handle = thread::spawn(move || {
        barrier_start_clone.wait();
        barrier_sync_clone.wait();
        let mut acc = acc_clone.lock().unwrap();
        acc.modify(20); // This will cause an underflow
    });

    barrier_start.wait();
    {
        let mut acc = account.lock().unwrap();
        acc.modify(5);
    }

    barrier_sync.wait();
    handle.join().unwrap();

    let acc = account.lock().unwrap();
    acc.retrieve()
}

fn main() {
    let final_value = run_app();
    println!("Final balance: {}", final_value);
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
