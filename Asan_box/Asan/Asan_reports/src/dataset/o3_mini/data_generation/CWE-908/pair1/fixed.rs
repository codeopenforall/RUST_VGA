use std::sync::Arc;
use std::thread;
struct Settings {
    timeout: u64,
    retries: u32,
}
impl Settings {
    fn load() -> Self {
        Settings {
            timeout: 30,
            retries: 3,
        }
    }
}
fn main() {
    let shared = Arc::new(Settings::load());
    let cloned = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        if cloned.retries > 0 {
            println!("Retries is set to {}", cloned.retries);
        } else {
            println!("Retries is zero, value: {}", cloned.retries);
        }
    });
    handle.join().unwrap();
    println!("Timeout is: {}", shared.timeout);
}


#[cfg(test)]
mod tests {
    // Import the module under test.
    // The following tests assume that the "load" function is available in the current scope.
    use super::*;

    #[test]
    fn test_configuration_integrity() {
        // For the fixed version, Settings::load() initializes timeout to 30 and retries to 3,
        // so the sum of these values should equal 33.
        // For the vulnerable version, retries is uninitialized leading to unpredictable outcomes.
        #[allow(unused_unsafe)]
        let config = unsafe { Settings::load() };
        let sum = config.timeout + config.retries as u64;
        assert_eq!(sum, 33, "Configuration fields did not sum to the expected value");
    }
}
