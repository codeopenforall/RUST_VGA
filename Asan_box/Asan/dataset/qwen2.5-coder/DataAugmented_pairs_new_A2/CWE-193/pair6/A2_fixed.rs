use std::vec::Vec;

struct Processor;

impl Processor {
    pub fn execute(&self, input: &[u32]) -> u32 {
        input.iter().sum()
    }
}

fn main() {
    let proc = Processor;
    let data = vec![1, 2, 3, 4];
    let result = proc.execute(&data);
    println!("Result: {}", result);
}