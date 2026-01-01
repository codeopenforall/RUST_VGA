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

//////////////////////
// Test Oracle
//////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    // The expected result is the sum of 100 (max_conn) and 443 (port), i.e., 543.
    // In the vulnerable code, max_conn is uninitialized which results in undefined behavior,
    // and the test is expected to fail. In the corrected version, the test should pass.
    #[test]
    fn test_compute() {
        let res = compute();
        assert_eq!(res, 543, "The computed value did not match the expected result.");
    }
}
