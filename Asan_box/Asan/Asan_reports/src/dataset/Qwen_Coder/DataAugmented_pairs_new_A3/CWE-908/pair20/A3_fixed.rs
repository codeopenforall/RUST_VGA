use std::mem::MaybeUninit;

struct Config {
    value: i32,
    message: String,
}

pub fn run_app() -> String {
    let mut config = MaybeUninit::<Config>::uninit();
    let config_ptr = config.as_mut_ptr();

    unsafe {
        (*config_ptr).value = 42;
        std::ptr::write(&mut (*config_ptr).message, String::from("Initialized!"));
    }

    let config = unsafe { config.assume_init() };
    config.message
}

#[cfg(test)]
mod tests {
    use super::run_app;
    
    #[test]
    fn oracle() {
        // The expected correct output is "Initialized!"
        let result = run_app();
        assert_eq!(
            result, 
            "Initialized!", 
            "Test failed: The configuration message did not match the expected initialized value."
        );
    }
}
