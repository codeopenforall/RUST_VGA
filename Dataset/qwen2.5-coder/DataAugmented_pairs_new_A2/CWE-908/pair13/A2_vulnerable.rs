use std::mem::MaybeUninit;
use std::thread;

struct Config {
    address: String,
    port_number: u16,
    connection_limit: usize,
}

impl Config {
    unsafe fn initialize(address: &str, port: u16) -> Self {
        let mut settings = MaybeUninit::<Config>::uninit();
        let ptr = settings.as_mut_ptr();
        (*ptr).address = address.to_owned();
        (*ptr).port_number = port;
        settings.assume_init()
    }

    fn evaluate(&self) -> usize {
        self.connection_limit + self.port_number as usize
    }
}

pub fn compute() -> usize {
    unsafe { Config::initialize("127.0.0.1", 443).evaluate() }
}

fn execute() {
    let result = compute();
    println!("Computed Result: {}", result);
}

fn main() {
    let thread_handle = thread::spawn(|| {
        execute();
    });
    thread_handle.join().unwrap();
}