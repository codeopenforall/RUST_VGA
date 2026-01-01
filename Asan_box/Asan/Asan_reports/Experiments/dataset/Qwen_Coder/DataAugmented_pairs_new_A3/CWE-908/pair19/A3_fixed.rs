pub struct Config {
    pub threshold: i32,
    pub factor: i32,
}

pub fn execute() -> i32 {
    let mut config = Config {
        threshold: 10,
        factor: 0,
    };

    // Simulate a pointer to the config
    let config_ptr: *mut Config = &mut config;

    // Safely write to the factor field using a pointer
    unsafe {
        std::ptr::write(&mut (*config_ptr).factor, 20);
    }

    config.threshold + config.factor
}

#[cfg(test)]
mod test_oracle {
    // Import the common interface function from the module.
    // Depending on whether the fixed or vulnerable version is compiled,
    // this test will pass only when both fields are correctly initialized.
    use super::execute;

    #[test]
    fn oracle() {
        // Expected sum is 10 (threshold) + 20 (factor) = 30.
        let result = execute();
        assert_eq!(result, 30, "Test oracle: Expected sum is 30");
    }
}
