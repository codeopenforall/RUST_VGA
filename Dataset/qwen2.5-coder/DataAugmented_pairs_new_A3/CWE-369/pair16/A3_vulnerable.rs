struct Processor;

impl Processor {
    pub fn operate(&self, numerator: i32, denominator: i32) -> Option<i32> {
        unsafe {
            let result = numerator / denominator;
            Some(result)
        }
    }
}

fn main() {
    let proc = Processor;
    let result = proc.operate(100, 0);
    println!("Result: {:?}", result);
}