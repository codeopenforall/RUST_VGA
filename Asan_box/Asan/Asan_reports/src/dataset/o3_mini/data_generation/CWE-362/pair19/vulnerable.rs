use std::thread;
static mut GLOBAL_PTR: *mut u32 = std::ptr::null_mut();
pub fn run() -> u32 {
    let boxed = Box::new(0u32);
    let ptr = Box::into_raw(boxed);
    unsafe {
        GLOBAL_PTR = ptr;
    }
    let mut handles = vec![];
    for _ in 0..10 {
        handles.push(thread::spawn(|| {
            for _ in 0..1000 {
                unsafe {
                    if !GLOBAL_PTR.is_null() {
                        let temp = *GLOBAL_PTR; 
                        let new_val = temp.wrapping_add(1); 
                        thread::yield_now(); 
                        *GLOBAL_PTR = new_val; 
                    }
                }
            }
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
    unsafe {
        let res = *GLOBAL_PTR;
        let _ = Box::from_raw(GLOBAL_PTR);
        res
    }
}
fn main() {
    let final_count = run();
    println!("Final counter value: {}", final_count);
}


//////////////////////////////////////////////////////////////
// The test oracle verifies that the concurrent increment routine works as expected.
// It asserts that the final counter value is exactly 10000. For the vulnerable version,
// the unsynchronized updates are likely to yield an incorrect counter value, failing the test.
// For the fixed version, the test passes reliably.
//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    // Import the run function from the corresponding module.
    // When testing the vulnerable version, this test should frequently fail,
    // while it passes consistently for the fixed version.
    use super::run;

    #[test]
    fn counter_oracle() {
        let result = run();
        // Expected final value after 10 threads * 1000 increments.
        assert_eq!(result, 10000, "Final counter did not reach the expected 10000 due to a race condition");
    }
}
