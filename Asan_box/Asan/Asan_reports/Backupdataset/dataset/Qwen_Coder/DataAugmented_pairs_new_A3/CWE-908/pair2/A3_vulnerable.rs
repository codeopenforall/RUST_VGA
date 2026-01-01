use std::mem::MaybeUninit;

#[derive(Debug)]
struct Config {
    threshold: u32,
    message: String,
}

impl Config {
    fn validate(&self) -> bool {
        self.threshold > 0 && !self.message.is_empty()
    }
}

fn load_configuration() -> Config {
    let mut storage: MaybeUninit<Config> = MaybeUninit::uninit();
    unsafe {
        let ptr = storage.as_mut_ptr();
        (*ptr).threshold = 100;
        // Intentionally omitting initialization of `message`
        storage.assume_init()
    }
}

fn main() {
    let config = load_configuration();
    println!("{:?}", config);
}