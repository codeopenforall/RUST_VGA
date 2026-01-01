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