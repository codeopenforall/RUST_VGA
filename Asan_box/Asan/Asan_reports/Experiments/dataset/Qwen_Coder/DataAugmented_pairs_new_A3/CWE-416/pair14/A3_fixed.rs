pub struct Data {
    pub value: i32,
}

pub fn process() -> i32 {
    let boxed_data = Box::new(Data { value: 42 });
    let raw_ptr = Box::into_raw(boxed_data);
    let result = unsafe { (*raw_ptr).value };
    let _ = unsafe { Box::from_raw(raw_ptr) };
    result
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
