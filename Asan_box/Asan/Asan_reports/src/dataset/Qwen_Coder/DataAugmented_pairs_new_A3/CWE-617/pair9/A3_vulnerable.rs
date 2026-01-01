struct Processor {
    data: Vec<u32>,
}

impl Processor {
    fn new(size: usize) -> Self {
        Processor {
            data: (0..size as u32).collect(),
        }
    }

    fn process(&self, index: usize, limit: u32) -> u32 {
        let val = self.data[index];
        assert!(val < limit, "Invariant violation: value {} is not less than {}", val, limit);
        val
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <index> <threshold>", args[0]);
        return;
    }

    let idx: usize = args[1].parse().unwrap();
    let threshold: u32 = args[2].parse().unwrap();

    let proc = Processor::new(10);
    let res = proc.process(idx, threshold);
    println!("Processed value: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test oracle uses an input that causes the vulnerable code to panic
    // while the fixed code—ignoring the external threshold—succeeds.
    #[test]
    fn contract_test() {
        let instance = Processor::new(10);
        // With index 5, the data value is 5.
        // In the vulnerable version, providing threshold '5' triggers the assertion (5 < 5 fails).
        // In the fixed version, the external threshold is ignored, and the safe internal limit (10) is used.
        let value = instance.process(5, 5);
        assert_eq!(value, 5, "Processing did not return the expected value.");
    }
}
