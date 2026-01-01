trait Calculation {
    fn evaluate(&self, numerator: i32, denominator: i32) -> i32;
}

struct Processor;

impl Calculation for Processor {
    fn evaluate(&self, numerator: i32, denominator: i32) -> i32 {
        if denominator == 0 {
            0
        } else {
            numerator / denominator
        }
    }
}

fn execute(numerator: i32, denominator: i32) -> i32 {
    let processor = Processor;
    processor.evaluate(numerator, denominator)
}

pub fn calculate(numerator: i32, denominator: i32) -> i32 {
    execute(numerator, denominator)
}