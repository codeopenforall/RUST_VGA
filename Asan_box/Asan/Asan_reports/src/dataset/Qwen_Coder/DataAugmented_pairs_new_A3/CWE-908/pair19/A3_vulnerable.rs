use std::mem::MaybeUninit;
use std::thread;
struct Settings {
    threshold: u32,
    factor: u32,
}
impl Settings {
    unsafe fn load() -> Settings {
        let mut config = MaybeUninit::<Settings>::uninit();
        let config_ptr = config.as_mut_ptr();
        std::ptr::write(&mut (*config_ptr).threshold, 10);
        // Introducing a bug by omitting the initialization of 'factor'
        // config.assume_init()
        std::ptr::read(config_ptr)
    }
}
pub fn execute() -> u32 {
    unsafe {
        let conf = Settings::load();
        let handle = thread::spawn(move || conf.threshold + conf.factor);
        handle.join().unwrap()
    }
}
fn main() {
    println!("Output: {}", execute());
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
