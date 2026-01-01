use std::env;
use std::thread;
static mut GLOBAL_VAR: i32 = 0;
const LIMIT: i32 = 1000;
pub fn run_app(increment: i32) {
    let mut threads = vec![];
    let num_threads = 10;
    for _ in 0..num_threads {
        let inc = increment;
        threads.push(thread::spawn(move || {
            for _ in 0..200 {
                unsafe {
                    GLOBAL_VAR = GLOBAL_VAR.wrapping_add(inc);
                }
            }
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    unsafe {
        assert!(GLOBAL_VAR < LIMIT, "Global variable exceeded safe limit!");
        println!("Completed run with GLOBAL_VAR value: {}", GLOBAL_VAR);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let increment = if args.len() > 1 {
        args[1].parse().unwrap_or(1)
    } else {
        1
    };
    run_app(increment);
}