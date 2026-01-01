struct Processor;

impl Processor {
    fn new() -> Self {
        Processor
    }

    fn process(&self, input: i32) -> i32 {
        let res: Result<i32, &'static str> = if input < 0 {
            Err("Negative input")
        } else {
            Ok(input * 2)
        };
        res.unwrap_or_default()
    }
}

fn main() {
    let processor = Processor::new();
    let result = processor.process(-5);
    println!("Result: {}", result);
}