use std::mem;

#[derive(Debug)]
struct Data {
    value: i32,
}

trait Action {
    fn execute(&self) -> i32;
}

struct Executor {
    pointer: *const Data,
}

impl Action for Executor {
    fn execute(&self) -> i32 {
        unsafe {
            (*self.pointer).value
        }
    }
}

pub fn compute() -> i32 {
    let data_instance = Box::new(Data { value: 42 });
    let raw_ptr = Box::into_raw(data_instance);
    let executor = Executor { pointer: raw_ptr };
    unsafe {
        let _freed = Box::from_raw(raw_ptr);
    }
    unsafe {
        let dummy = Box::new(Data { value: 0 });
        mem::forget(dummy);
    }
    let result = executor.execute();
    result
}

fn main() {
    let res = compute();
    println!("Computed result: {}", res);
}

/*
   The following test oracle verifies that the computation result is 42. For the vulnerable version,
   due to the use-after-free, the result is likely to be incorrect (or could cause a panic), causing the test
   to fail. In contrast, the corrected version returns the expected value, making the test pass.
*/

#[cfg(test)]
mod tests {
    // Change the import path as needed to point to the module under test.
    use super::compute;

    #[test]
    fn test_compute_result() {
        let res = compute();
        // Assert that the computed value is as expected.
        assert_eq!(res, 42, "Test failed: expected 42, got {}", res);
    }
}
