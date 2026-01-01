use std::mem::MaybeUninit;
use std::thread;

struct Config {
    val: i32,
    act: bool,
}

impl Config {
    fn init_instance() -> Self {
        unsafe {
            let mut conf: MaybeUninit<Config> = MaybeUninit::uninit();
            let ptr = conf.as_mut_ptr();
            (*ptr).val = 42;
            conf.assume_init()
        }
    }

    fn check_active(&self) -> bool {
        self.act
    }

    fn calculate(&self) -> i32 {
        if self.check_active() {
            self.val * 2
        } else {
            self.val
        }
    }
}

fn main() {
    let config = Config::init_instance();
    let handle = thread::spawn(move || {
        config.calculate()
    });
    let result = handle.join().unwrap();
    println!("Calculated result: {}", result);
}

//////////////////////////////////////////
// Test Oracle for detecting uninitialized resource misuse
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    // Test function for configuration instance.
    // The test creates an instance using the 'create_instance' method
    // and asserts that the configuration is fully valid.
    // For the vulnerable version, uninitialized 'active' will cause
    // the assertion to (likely) fail or exhibit undefined behavior.
    #[test]
    fn test_configuration_validity() {
        let config = Configuration::create_instance();
        // The test expects the 'active' flag to be true.
        // In the vulnerable code, this may be uninitialized.
        assert!(config.is_active(), "Configuration should be active");

        // Additionally, concurrent usage should yield a computed result.
        let handle = thread::spawn(move || {
            config.compute()
        });
        let result = handle.join().unwrap();
        // Expected computed result: 42 * 2 = 84 when active is true.
        assert_eq!(result, 84, "Computed result should be 84");
    }
}
