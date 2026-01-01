use std::mem::MaybeUninit;
use std::thread;

struct Config {
    limit: u32,
    multiplier: u32,
}

impl Config {
    unsafe fn initialize() -> Config {
        let mut settings = MaybeUninit::<Config>::uninit();
        let settings_ptr = settings.as_mut_ptr();
        std::ptr::write(&mut (*settings_ptr).limit, 10);
        settings.assume_init()
    }
}

pub fn execute() -> u32 {
    unsafe {
        let params = Config::initialize();
        let handle = thread::spawn(move || params.limit + params.multiplier);
        handle.join().unwrap()
    }
}

fn main() {
    println!("Result: {}", execute());
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
