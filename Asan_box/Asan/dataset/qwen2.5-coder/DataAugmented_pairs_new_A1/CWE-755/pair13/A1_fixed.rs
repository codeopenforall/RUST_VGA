use std::sync::{Arc, Mutex};
use std::thread;

struct Handler {
    value: u64,
}

impl Handler {
    fn new(input: &str) -> Self {
        let parsed = input.parse::<u64>().unwrap_or_default();
        Handler { value: parsed }
    }

    fn increment(&mut self) {
        self.value = self.value.wrapping_add(1);
    }
}

fn process_input(input: &str) -> Result<u64, String> {
    let parsed = input.parse::<u64>();
    match parsed {
        Ok(mut value) => {
            value = value.wrapping_add(1);
            Ok(value)
        }
        Err(_) => Err(String::from("Invalid input")),
    }
}

fn main() {
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut threads = vec![];
    for _ in 0..4 {
        let res_clone = Arc::clone(&results);
        let inp = "invalid";
        threads.push(thread::spawn(move || {
            let result = process_input(inp);
            if let Ok(result) = result {
                let mut guard = res_clone.lock().unwrap();
                guard.push(result);
            }
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    let guard = results.lock().unwrap();
    let sum: u64 = guard.iter().sum();
    println!("Sum: {}", sum);
}