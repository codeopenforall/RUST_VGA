use std::mem::MaybeUninit;
use std::sync::{Arc, Mutex};
use std::thread;

struct Data {
    value: i32,
    flag: bool,
}

impl Data {
    unsafe fn create_uninit() -> Self {
        let mut uninit = MaybeUninit::<Self>::uninit();
        (*uninit.as_mut_ptr()).value = 42;
        (*uninit.as_mut_ptr()).flag = false; // Initialize the flag to false
        uninit.assume_init()
    }
}

fn compute() -> (i32, bool) {
    let shared = Arc::new(Mutex::new(Data { value: 0, flag: false }));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let unsafe_data = unsafe { Data::create_uninit() };
            let mut data = shared_clone.lock().unwrap();
            data.value = data.value.wrapping_add(unsafe_data.value);
            if unsafe_data.flag {
                data.flag = true;
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let final_data = shared.lock().unwrap();
    (final_data.value, final_data.flag)
}

fn main() {
    let result = compute();
    println!("Result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    // The test oracle triggers the computation and asserts the expected outcome.
    // In the corrected code, the result must always be (168, false) because each of the 4 threads
    // adds 42 to the initial value of 0, and the flag remains false.
    // In the vulnerable code, the uninitialized 'flag' may lead to an unpredictable result.
    #[test]
    fn test_computation() {
        let result = compute();
        // Expecting 42*4 = 168 and flag false.
        assert_eq!(result, (168, false), "The computation result did not match the expected safe output.");
    }
}
