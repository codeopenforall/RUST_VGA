use std::env;
use std::thread;
use std::slice;
use std::str;
struct Data {
    input: String,
}
trait Executor {
    fn execute(&self) -> String;
}
impl Executor for Data {
    fn execute(&self) -> String {
        // Instead of using format! with user input, return the input as is.
        self.input.clone()
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: program <format-string>");
        return;
    }
    let data = Data { input: args[1].clone() };
    let handle = thread::spawn(move || data.execute());
    let res = handle.join().unwrap();
    println!("{}", res);
}