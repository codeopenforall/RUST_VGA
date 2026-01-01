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

fn retrieve_config() -> Option<&'static ConfigStruct> {
    unsafe {
        if GLOBAL_SETTING.is_null() {
            None
        } else {
            Some(&*GLOBAL_SETTING)
        }
    }
}

fn compute(input: u32) -> Result<u32, &'static str> {
    match retrieve_config() {
        Some(config) => {
            if config.value == 0 {
                Err("Invalid configuration: value cannot be zero")
            } else {
                Ok(input / config.value)
            }
        }
        None => Err("Configuration not initialized"),
    }
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