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