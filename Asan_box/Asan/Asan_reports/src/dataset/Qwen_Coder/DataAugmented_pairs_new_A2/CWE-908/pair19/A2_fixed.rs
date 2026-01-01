use std::mem::MaybeUninit;
use std::thread;

struct Settings {
    threshold: u32,
    factor: u32,
}

impl Settings {
    unsafe fn load() -> Settings {
        let mut config = MaybeUninit::<Settings>::uninit();
        std::ptr::write(config.as_mut_ptr(), Settings { threshold: 10, factor: 20 });
        config.assume_init()
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
