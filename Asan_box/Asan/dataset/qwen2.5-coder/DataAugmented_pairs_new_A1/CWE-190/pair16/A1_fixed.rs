struct MultiplierOp {
    multiplier: u32,
}

impl MultiplierOp {
    fn compute(&self, input: u32) -> u64 {
        let a = input as u64;
        let b = self.multiplier as u64;
        a * b
    }
}

fn main() {
    let op = MultiplierOp { multiplier: 2 };
    let op_clone = op.clone();
    let test_input: u32 = 50_000;
    let result = op_clone.compute(test_input);
    println!("{}", result);
}