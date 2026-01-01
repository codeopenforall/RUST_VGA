use std::mem::MaybeUninit;
use std::sync::{Arc, Barrier};
use std::thread;
#[derive(Debug)]
struct Data {
    value: i32,
}
impl Data {
    unsafe fn new_uninit() -> Data {
        let config: MaybeUninit<Data> = MaybeUninit::uninit();
        config.assume_init()
    }
}
pub fn execute() -> i32 {
    let barrier = Arc::new(Barrier::new(2));
    let shared = Arc::new(unsafe { Data::new_uninit() });
    let barrier_clone = Arc::clone(&barrier);
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        barrier_clone.wait();
        let read_val = unsafe { shared_clone.value };
        read_val
    });
    barrier.wait();
    let ptr = Arc::as_ptr(&shared) as *mut Data;
    unsafe {
        (*ptr).value = 100;
    }
    handle.join().unwrap()
}
fn main() {
    let res = execute();
    println!("Result: {}", res);
}


#[cfg(test)]
mod tests {
    // Import the execute function from the module (it should be available in both vulnerable and fixed builds).
    use super::execute;

    #[test]
    fn test_oracle() {
        // When the code is fixed, execute() must consistently return 100.
        // In the vulnerable variant, due to the race and uninitialized memory, this may not hold.
        let res = execute();
        assert_eq!(res, 100, "Expected the result to be 100, but got {}", res);
    }
}
