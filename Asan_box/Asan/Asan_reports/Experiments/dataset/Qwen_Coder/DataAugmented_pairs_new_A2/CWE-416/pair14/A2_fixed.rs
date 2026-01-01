use std::boxed::Box;

struct Resource {
    value: i32,
}

impl Resource {
    fn new(v: i32) -> Self {
        Resource { value: v }
    }
}

fn process() -> i32 {
    let resource = Box::new(Resource::new(42));
    let raw_ptr: *mut Resource = Box::into_raw(resource);
    let result = unsafe { (*raw_ptr).value };
    let _ = unsafe { Box::from_raw(raw_ptr) };
    result
}

fn main() {
    let result = process();
    println!("Result: {}", result);
}

/*
   Test Oracle:
   This unit test invokes the "process" function and asserts that it returns the expected value (42).
   In the vulnerable version, due to use‐after‐free, undefined behavior may cause the result 
   to deviate from 42 or even panic, causing the test to fail. In the fixed version, the test passes.
*/

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process_value() {
        // The trigger value 42 is used to test boundary/integrity.
        let result = process();
        assert_eq!(result, 42, "Process function did not return the expected value 42.");
    }
}
