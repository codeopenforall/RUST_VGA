use std::sync::{Arc, Mutex};
use std::thread;

struct Account {
    funds: i32,
}

impl Account {
    fn initialize() -> Self {
        Account { funds: 0 }
    }
    fn deposit(&mut self, sum: i32) -> Result<(), &'static str> {
        unsafe {
            let pointer = &mut self.funds as *mut i32;
            *pointer += sum;
        }
        Ok(())
    }
    fn check_balance(&self) -> i32 {
        self.funds
    }
}

fn run_transactions() -> Result<i32, &'static str> {
    let account = Arc::new(Mutex::new(Account::initialize()));
    let mut threads = vec![];
    let rounds = 10;
    let num_threads = 10;
    for _ in 0..num_threads {
        let account_clone = Arc::clone(&account);
        threads.push(thread::spawn(move || {
            for _ in 0..rounds {
                let mut acc = account_clone.lock().unwrap();
                acc.deposit(300_000_000).unwrap();
            }
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    let total_funds = account.lock().unwrap().check_balance();
    Ok(total_funds)
}

fn main() {
    match run_transactions() {
        Ok(value) => println!("Total funds: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}