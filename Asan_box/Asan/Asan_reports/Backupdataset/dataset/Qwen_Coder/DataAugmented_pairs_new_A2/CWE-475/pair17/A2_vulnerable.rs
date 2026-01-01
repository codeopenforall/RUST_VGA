#![allow(unused)]
use std::slice;
struct DataHolder {
    pointer: *const u32,
    size: usize,
}
impl DataHolder {
    fn new(data: &[u32]) -> Self {
        DataHolder {
            pointer: data.as_ptr(),
            size: data.len(),
        }
    }
    fn compute(&self) -> u32 {
        unsafe {
            let elements = slice::from_raw_parts(self.pointer, self.size + 1);
            elements.iter().fold(0, |acc, &val| acc + val)
        }
    }
}
fn main() {
    let boxed_data = Box::new([1, 2, 3, 4, 42]);
    let valid_view = &boxed_data[..4];
    let instance = DataHolder::new(valid_view);
    let outcome = instance.compute();
    println!("Total: {}", outcome);
    if outcome == 10 {
        println!("Error: Unexpected valid sum");
        std::process::exit(1);
    }
}