use std::sync::Arc;
use std::thread;

struct Processor {}

impl Processor {
    fn new() -> Processor {
        Processor {}
    }

    fn process(&self, user_input: &str) -> String {
        // Ensure the input is treated as a literal string
        user_input.to_string()
    }
}

fn main() {
    let proc = Arc::new(Processor::new());
    let proc_clone = Arc::clone(&proc);
    let handle = thread::spawn(move || {
        let res = proc_clone.process("%x %x %x");
        println!("{}", res);
    });
    handle.join().unwrap();
}