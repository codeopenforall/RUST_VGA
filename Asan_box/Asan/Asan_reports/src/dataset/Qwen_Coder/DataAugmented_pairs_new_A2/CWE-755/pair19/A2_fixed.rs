use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

struct Config {
    secret: i32,
}

impl Config {
    fn load() -> Result<Self, &'static str> {
        let value = env::var("CONFIG_VAL").map_err(|_| "missing CONFIG_VAL")?;
        let secret = value.parse::<i32>().map_err(|_| "CONFIG_VAL not an integer")?;
        if secret == 0 {
            return Err("CONFIG_VAL cannot be zero");
        }
        Ok(Config { secret })
    }
}

fn compute(input: i32, config: &Config) -> i32 {
    input / config.secret
}

fn execute() -> Result<(), &'static str> {
    let config = Config::load()?;
    let shared = Arc::new(Mutex::new(config));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let conf = shared_clone.lock().unwrap();
        compute(100, &conf)
    });
    let _res = handle.join().map_err(|_| "Thread panicked")?;
    println!("Execution completed without panic.");
    Ok(())
}

fn main() {
    if let Err(e) = execute() {
        eprintln!("Error encountered: {}", e);
        std::process::exit(1);
    }
}

////////////////////////////////////////////
// Test Oracle (Unit Test)
////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // The oracle test verifies that the application does not panic when configuration errors
    // are properly handled. It removes CONFIG_VAL so that the vulnerable version (if present)
    // would panic due to division by zero, whereas the fixed version returns an error gracefully.
    #[test]
    fn test_oracle() {
        // Ensure the environment variable is not set.
        std::env::remove_var("CONFIG_VAL");
        
        // Execute in a catch_unwind block to detect panics.
        let result = panic::catch_unwind(|| {
            // In the fixed version, execute() returns a Result and does not panic.
            // In the vulnerable version, execute() will panic due to a division by zero.
            execute();
        });
        assert!(
            result.is_ok(),
            "Application panicked, indicating improper handling of exceptional conditions."
        );
    }
}
