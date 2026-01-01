use std::error::Error;
use std::fs;
use std::ptr;

struct Settings {
    factor: u32,
}

impl Settings {
    fn load() -> Result<Self, Box<dyn Error>> {
        let data = fs::read_to_string("config.txt")?;
        let factor = data.trim().parse::<u32>()?;
        Ok(Settings { factor })
    }
}

fn process(input: u32) -> Result<u32, Box<dyn Error>> {
    let settings = Settings::load()?;
    let ptr = &settings.factor as *const u32;
    unsafe {
        Ok(*ptr.offset(0))
    }
}

fn execute(input: u32) -> Result<u32, Box<dyn Error>> {
    process(input)
}

fn main() {
    let handle = std::thread::spawn(|| {
        let computed = process(5).expect("Processing failed");
        println!("Computed value: {}", computed);
    });
    handle.join().expect("Thread panicked");
}