use std::mem::MaybeUninit;
use std::sync::{Arc, Mutex};
mod app {
    use super::*;
    pub struct Item {
        secret: u32,
        public: u32,
    }
    impl Item {
        pub fn new(flag: bool) -> Self {
            let mut uninit = MaybeUninit::<Item>::uninit();
            unsafe {
                let ptr = uninit.as_mut_ptr();
                (*ptr).secret = 42;
                if flag {
                    (*ptr).public = 128;
                } else {
                    (*ptr).public = 128;
                }
                uninit.assume_init()
            }
        }
        pub fn sum(&self) -> u32 {
            self.secret + self.public
        }
    }
    pub fn execute() {
        let item = Item::new(false);
        let shared = Arc::new(Mutex::new(item));
        let shared_clone = Arc::clone(&shared);
        let handle = std::thread::spawn(move || {
            let obj = shared_clone.lock().unwrap();
            obj.sum()
        });
        let sum_main = {
            let obj = shared.lock().unwrap();
            obj.sum()
        };
        let sum_thread = handle.join().unwrap();
        println!("Sum from main: {}, Sum from thread: {}", sum_main, sum_thread);
    }
}
fn main() {
    app::execute();
}


#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    // The test assumes that the public API provided by module app is available.
    // For both versions, we require that new(false) returns an Item whose sum() equals 170.
    // In the vulnerable version, reading uninitialized memory typically results in an incorrect sum.
    // In the corrected version, the sum is correctly computed.
    use crate::app::Item;

    #[test]
    fn test_sum() {
        let item = Item::new(false);
        let expected = 170; // 42 (secret) + 128 (public)
        let shared = Arc::new(Mutex::new(item));
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let it = shared_clone.lock().unwrap();
            it.sum()
        });
        let sum_main = {
            let it = shared.lock().unwrap();
            it.sum()
        };
        let sum_thread = handle.join().unwrap();

        assert_eq!(sum_main, expected, "Main thread computed incorrect sum");
        assert_eq!(sum_thread, expected, "Spawned thread computed incorrect sum");
    }
}
