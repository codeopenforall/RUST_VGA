struct Data {
    value: i32,
}

impl Data {
    fn new(value: i32) -> Self {
        Data { value }
    }
}

pub fn compute() -> i32 {
    let boxed = Box::new(Data::new(42));
    let raw_ptr = Box::into_raw(boxed);
    unsafe {
        Box::from_raw(raw_ptr);
        let _dummy = Box::new(Data::new(100));
        (*raw_ptr).value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This function serves as the test oracle.
    // When linked with the vulnerable version, it is expected to yield a result different from 42,
    // triggering the assertion failure (indicative of a use-after-free error).
    // With the fixed version, compute() correctly returns 42.
    fn test_oracle() {
        let result = compute();
        assert_eq!(result, 42, "Unexpected result detected (use-after-free vulnerability).");
    }

    #[test]
    fn oracle_test() {
        test_oracle();
    }
}
