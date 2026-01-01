use std::fs;
use std::thread;

struct Settings {
    factor: u32,
}

impl Settings {
    fn load() -> Result<Self, &'static str> {
        let data = fs::read_to_string("config.txt").map_err(|_| "Failed to read config file")?;
        let factor = data.trim().parse::<u32>().map_err(|_| "Invalid configuration")?;
        Ok(Settings { factor })
    }
}

fn process(input: u32) -> Result<u32, &'static str> {
    let settings = Settings::load()?;
    let result = input.wrapping_mul(settings.factor);
    Ok(result)
}

fn execute(input: u32) -> Result<u32, &'static str> {
    let result = process(input)?;
    Ok(result)
}

fn main() {
    let handle = thread::spawn(|| {
        let computed = process(5).expect("Failed to compute result");
        println!("Result: {}", computed);
    });
    handle.join().unwrap();
}