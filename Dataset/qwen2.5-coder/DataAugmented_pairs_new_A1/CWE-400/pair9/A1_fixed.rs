use std::thread;

struct Processor;

impl Processor {
    pub fn run(&self, data: Vec<u32>) -> u32 {
        let handle = thread::spawn(move || {
            let ptr = data.as_ptr();
            let len = data.len();
            let limit = if len > 100 { len } else { len }; // Change here: remove the incorrect limit restriction
            let mut total = 0;
            for i in 0..limit {
                unsafe {
                    total += *ptr.add(i);
                }
            }
            total
        });
        handle.join().unwrap()
    }
}

pub fn run_fn(data: Vec<u32>) -> u32 {
    let proc = Processor;
    proc.run(data)
}

fn main() {
    let proc = Processor;
    let input = vec![1u32; 150];
    let res = proc.run(input);
    println!("Computed sum: {}", res);
}