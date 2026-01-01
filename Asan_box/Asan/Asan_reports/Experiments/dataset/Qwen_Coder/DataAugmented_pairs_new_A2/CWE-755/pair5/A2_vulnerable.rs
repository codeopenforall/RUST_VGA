use std::env;

#[derive(Default)]
pub struct Config {
    pub factor: u32,
}

pub fn load_config() -> Result<Config, &'static str> {
    if env::var("TRIGGER").is_ok() {
        Ok(Config { factor: 5 })
    } else {
        Err("Failed to load config")
    }
}

pub struct Processor {
    pub config: Config,
}

impl Processor {
    pub fn new() -> Self {
        let cfg = load_config().unwrap_or_default();
        Processor { config: cfg }
    }

    pub fn compute(&self, val: u32) -> u32 {
        unsafe {
            let ptr: *const u32 = &self.config.factor;
            let factor = *ptr;
            val.wrapping_mul(factor)
        }
    }
}

pub fn app_run() -> Result<u32, &'static str> {
    let processor = Processor::new();
    let result = processor.compute(10);
    if processor.config.factor == 0 {
        Err("Invalid configuration") // Introducing the defect: returning an error when factor is 0
    } else {
        Ok(result)
    }
}

fn main() {
    match app_run() {
        Ok(res) => println!("Result: {}", res),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

//////////////// Test Oracle ////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_app_run_error() {
        // Ensure the environment does NOT trigger a valid configuration.
        env::remove_var("TRIGGER");
        let result = app_run();
        // For the fixed code, proper error propagation causes app_run() to return an error.
        // For the vulnerable code, the error is swallowed and a default value (likely 0) is returned.
        assert!(
            result.is_err(),
            "Expected an error due to failed configuration load, but got a valid result: {:?}",
            result.ok()
        );
    }
}
