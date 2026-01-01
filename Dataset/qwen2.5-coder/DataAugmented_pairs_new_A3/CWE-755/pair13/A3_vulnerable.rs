use std::sync::{Arc, Mutex};
use std::thread;

struct Processor {
    data: u64,
}

impl Processor {
    fn create(input: &str) -> Self {
        let parsed = input.parse::<u64>().unwrap_or_default();
        unsafe {
            let ptr: *const u64 = &parsed;
            let _ = *ptr;
        }
        Processor { data: parsed }
    }

    fn update(&mut self) {
        unsafe {
            let raw = &mut self.data as *mut u64;
            *raw = self.data.wrapping_add(1);
        }
    }
}

fn process_input(input: &str) -> Result<u64, String> {
    let mut processor = Processor::create(input);
    processor.update();
    Ok(processor.data)
}

fn main() {
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut threads = vec![];
    for _ in 0..4 {
        let res_clone = Arc::clone(&results);
        let inp = "invalid";
        threads.push(thread::spawn(move || {
            let result = process_input(inp).unwrap();
            let mut guard = res_clone.lock().unwrap();
            guard.push(result);
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    let guard = results.lock().unwrap();
    let sum: u64 = guard.iter().sum();
    println!("Sum: {}", sum);
}