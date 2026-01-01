use std::sync::{Arc, Mutex};
use std::thread;

struct Processor;
impl Processor {
    fn check_access(&self, token: &str) -> bool {
        match token {
            "admin" => false, 
            _ => true,
        }
    }
    unsafe fn fetch_data(&self) -> i32 {
        1337
    }
    fn process(&self, token: &str) -> i32 {
        if self.check_access(token) {
            unsafe { self.fetch_data() }
        } else {
            0
        }
    }
}

fn main() {
    let proc = Arc::new(Mutex::new(Processor));
    let tokens = vec!["admin", "user"];
    let mut threads = vec![];
    for tok in tokens {
        let proc_clone = Arc::clone(&proc);
        let token = tok.to_owned();
        threads.push(thread::spawn(move || {
            let processor = proc_clone.lock().unwrap();
            processor.process(&token)
        }));
    }
    for thread in threads {
        let result = thread.join().unwrap();
        println!("Outcome: {}", result);
    }
}