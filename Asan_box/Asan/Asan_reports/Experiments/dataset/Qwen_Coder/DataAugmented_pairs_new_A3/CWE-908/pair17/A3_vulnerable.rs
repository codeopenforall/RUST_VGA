use std::thread;

struct Config {
    name: String,
    timeout: u32, 
}

impl Config {
    fn construct() -> Self {
        Config {
            name: String::from("example"),
            timeout: 0, // Introduce uninitialized value for CWE-908
        }
    }
}

fn load_config() -> Config {
    Config::construct()
}

fn execute(config: Config) -> u32 {
    let handle = thread::spawn(move || {
        config.timeout.wrapping_add(1)
    });
    handle.join().unwrap()
}

fn main() {
    let cfg = load_config();
    let outcome = execute(cfg);
    println!("Outcome: {}", outcome);
}

#[cfg(test)]
mod tests {
    use super::*;

    // The test oracle expects the configuration's timeout to be exactly 30 so that
    // execute(load_config()) returns 30 + 1 == 31. In the vulnerable version, the uninitialized
    // timeout field causes undefined behavior and the assertion fails.
    #[test]
    fn config_timeout_must_be_correct() {
        let cfg = load_config();
        let result = execute(cfg);
        assert_eq!(result, 31, "The timeout has not been properly initialized; expected 31.");
    }
}
