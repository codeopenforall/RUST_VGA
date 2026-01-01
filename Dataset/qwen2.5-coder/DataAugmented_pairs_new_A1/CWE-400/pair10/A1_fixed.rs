use std::env;
use std::thread;
use std::sync::{Arc, Mutex};

const MAX_THREADS: u64 = 1000;

static mut GLOBAL: u64 = 0;

pub fn compute(n: u64) -> u64 {
    unsafe {
        GLOBAL = 0;
    }

    let n = if n > MAX_THREADS { MAX_THREADS } else { n };

    let mut handles = vec![];
    let global = Arc::new(Mutex::new(0));

    for _ in 0..n {
        let global_clone = Arc::clone(&global);
        handles.push(thread::spawn(move || {
            let mut val = global_clone.lock().unwrap();
            let _dummy = (0..1000).fold(0, |acc, x| acc + x);
            *val += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe { GLOBAL = *global.lock().unwrap() };
    unsafe { GLOBAL }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args.get(1)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
    let result = compute(n);
    println!("Result: {}", result);
}