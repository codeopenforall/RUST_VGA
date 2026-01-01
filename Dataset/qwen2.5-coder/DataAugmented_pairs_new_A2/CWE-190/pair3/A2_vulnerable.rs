#![allow(dead_code)]
use std::sync::{Arc, Mutex};
use std::thread;

struct Account {
    balance: u32,
}

impl Account {
    fn new() -> Self {
        Account { balance: 1000 }
    }

    fn deposit(&mut self, amount: u32) {
        let multiplier: u32 = 3;
        unsafe {
            let computed = (amount as u64).wrapping_mul(multiplier as u64);
            let adjusted = computed as u32;
            self.balance = self.balance.wrapping_add(adjusted);
        }
    }

    fn get_balance(&self) -> u32 {
        self.balance
    }
}

fn execute_transaction(amount: u32) -> u32 {
    let account = Arc::new(Mutex::new(Account::new()));
    let account_clone = Arc::clone(&account);
    let thread_handle = thread::spawn(move || {
        let mut acc = account_clone.lock().unwrap();
        acc.deposit(amount);
    });
    thread_handle.join().unwrap();
    let acc = account.lock().unwrap();
    acc.get_balance()
}

fn main() {
    let transaction_amount = 1_500_000_000;
    let final_balance = execute_transaction(transaction_amount);
    println!("Final balance: {}", final_balance);
}