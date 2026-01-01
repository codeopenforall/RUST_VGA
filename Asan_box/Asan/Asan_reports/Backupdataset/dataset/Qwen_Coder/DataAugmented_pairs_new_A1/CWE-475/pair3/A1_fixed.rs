use std::thread;

struct Processor;

impl Processor {
    fn operate(&self, input: &[u32]) -> Option<u32> {
        if input.is_empty() {
            None
        } else {
            Some(input[0])
        }
    }
}

fn main() {
    let proc_inst = Processor;
    let data: Vec<u32> = vec![];
    let handle = thread::spawn(move || {
        proc_inst.operate(&data)
    });
    match handle.join().unwrap() {
        Some(val) => println!("Output: {}", val),
        None => println!("No output produced"),
    }
}