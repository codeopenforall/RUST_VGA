struct Processor {
    data: Vec<u32>,
}

impl Processor {
    fn new(size: usize) -> Self {
        Processor {
            data: (0..size as u32).collect(),
        }
    }

    fn process(&self, index: usize, _limit: u32) -> u32 {
        let safe_limit = self.data.len() as u32;
        let val = index as u32;
        assert!(val < safe_limit, "Invariant violation: value {} is not less than {}", val, safe_limit);
        self.data[index]
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <index>", args[0]);
        return;
    }

    let idx: usize = args[1].parse().expect("Please provide a valid index");
    let instance = Processor::new(10);
    let res = instance.process(idx, 0);
    println!("Processed value: {}", res);
}