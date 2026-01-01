use std::{thread, ptr};

struct Data {
    count: u32,
}

static mut GLOBAL: *mut Data = ptr::null_mut();

fn run_system() -> u32 {
    unsafe {
        let data = Box::new(Data { count: 0 });
        GLOBAL = Box::into_raw(data);
    }

    let mut threads = vec![];
    for _ in 0..10 {
        threads.push(thread::spawn(|| {
            for _ in 0..100 {
                unsafe {
                    if !GLOBAL.is_null() {
                        let current = (*GLOBAL).count;
                        thread::yield_now();
                        (*GLOBAL).count = current + 1;
                    }
                }
            }
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    let result;
    unsafe {
        result = (*GLOBAL).count;
        let _ = Box::from_raw(GLOBAL);
        GLOBAL = ptr::null_mut();
    }
    result
}

fn main() {
    let total = run_system();
    println!("Final count: {}", total);
}

//////////////////////////////// Test Oracle //////////////////////////////////
#[cfg(test)]
mod tests {
    // Import run_system from the current module. When linking against the vulnerable build, this test should fail,
    // while linking against the fixed build, the test should pass.
    use super::run_system;

    #[test]
    fn oracle_test() {
        // The expected final count should be 1000 (10 threads * 100 increments each).
        let result = run_system();
        assert_eq!(result, 1000, "Final count did not match expected value! Race condition exists.");
    }
}
