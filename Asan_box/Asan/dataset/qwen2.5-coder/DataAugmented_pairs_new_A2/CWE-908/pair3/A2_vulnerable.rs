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