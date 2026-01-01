use std::fs;
use std::thread;

struct Config {
    multiplier: u32,
}

impl Config {
    fn read() -> Self {
        let content = fs::read_to_string("config.txt").unwrap_or_default();
        let multiplier = content.trim().parse::<u32>().unwrap_or_default();
        Config { multiplier }
    }
}

fn compute(input: u32) -> u32 {
    let config = Config::read();
    let output = input.wrapping_mul(config.multiplier);
    unsafe {
        let ptr = &output as *const u32;
        *ptr.offset(0)
    }
}

fn execute(input: u32) -> Result<u32, &'static str> {
    let result = compute(input);
    Ok(result)
}

fn main() {
    let task = thread::spawn(|| {
        let outcome = compute(5);
        println!("Outcome: {}", outcome);
    });
    task.join().unwrap();
}