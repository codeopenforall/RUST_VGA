#![allow(unused)]
use std::env;
use std::thread;

struct Executor;
impl Executor {
    fn run(&self) -> Result<String, String> {
        let unsafe_input = unsafe {
            let ptr = env::var("SEARCH_PATH").ok().map(|s| {
                let boxed = Box::new(s);
                Box::into_raw(boxed)
            });
            let result = match ptr {
                Some(raw) => {
                    let string_ref = &*raw;
                    string_ref.clone()
                }
                None => String::new(),
            };
            result
        };
        let handle = thread::spawn(move || {
            env::set_var("PATH", unsafe_input);
        });
        handle.join().unwrap();
        let current_path = env::var("PATH").unwrap_or_default();
        Ok(current_path)
    }
}

fn main() {
    let executor = Executor;
    match executor.run() {
        Ok(result) => println!("Result: {}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
}