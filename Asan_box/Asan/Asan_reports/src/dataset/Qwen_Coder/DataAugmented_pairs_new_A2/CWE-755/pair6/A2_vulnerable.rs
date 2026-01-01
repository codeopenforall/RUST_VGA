use std::thread;

struct ConfigStruct {
    value: u32,
}

static mut GLOBAL_SETTING: *mut ConfigStruct = std::ptr::null_mut();

fn read_config(input: &str) -> *mut ConfigStruct {
    let num = input.trim().parse::<u32>().unwrap_or_default();
    let config = Box::new(ConfigStruct { value: num });
    Box::into_raw(config)
}

fn retrieve_config() -> u32 {
    unsafe {
        if GLOBAL_SETTING.is_null() {
            return 0;
        }
        (*GLOBAL_SETTING).value
    }
}

fn compute(input: u32) -> Result<u32, &'static str> {
    let config_val = retrieve_config();
    if config_val == 0 {
        return Ok(input);
    }
    Ok(input / config_val)
}

fn execute() {
    unsafe {
        GLOBAL_SETTING = read_config("not_a_number");
    }
    let handle = thread::spawn(|| {
        compute(100).unwrap_or_default()
    });
    let result = handle.join().unwrap();
    println!("Result: {}", result);
}

fn main() {
    execute();
}

////////////////////////////////////////////
// Test Oracle for Error Handling in Configuration
//
// The oracle function below is used to verify the behavior of the
// two versions. When provided an invalid configuration input, the
// secure version should signal an error (by returning an Err) while
// the problematic version would erroneously compute a value.
//
// Note: This oracle is used as part of an automated unit test.
////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    // Setup a function to initialize the global configuration.
    // This simulates injecting an invalid configuration.
    fn init_invalid_config() {
        unsafe {
            // For testing, we simulate invalid input. In the vulnerable version,
            // this would set a default value silently.
            GLOBAL_SETTING = {
                // In the vulnerable version "read_config" always produces a config,
                // even when given invalid input.
                // In the secure version, this line would instead return an error.
                let config = Box::new(ConfigStruct { value: 0 });
                Box::into_raw(config)
            };
        }
    }

    #[test]
    fn test_invalid_configuration() {
        // Prepare the invalid configuration
        init_invalid_config();
        // The contract: Given an invalid configuration, the secure version should
        // propagate an error. In a correct setup the compute function should return Err.
        match compute(100) {
            Ok(result) => {
                // If the secure version were faulty, it would compute a result.
                // Thus, the test fails if a valid result is returned.
                assert!(false, "Expected an error from compute(), but got result: {}", result);
            },
            Err(e) => {
                // Correct behavior for the secure version.
                assert_eq!(e, "Invalid configuration: value cannot be zero");
            }
        }
    }
}
