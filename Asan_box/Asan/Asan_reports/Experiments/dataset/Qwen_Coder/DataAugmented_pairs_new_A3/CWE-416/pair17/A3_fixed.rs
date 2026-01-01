struct Data {
    value: i32,
}

impl Data {
    fn new(value: i32) -> Self {
        Data { value }
    }
}

pub fn calculate() -> i32 {
    let boxed = Box::new(Data::new(42));
    let ptr = Box::into_raw(boxed);
    let result = unsafe { (*ptr).value };
    // Ensure the memory is properly freed
    drop(unsafe { Box::from_raw(ptr) });
    result
}

/////////////////////////////////////////////////////////////////
// Test Oracle Function
// This test calls the calculate() function and asserts that the result 
// is 42. The vulnerable version returns 999 (or undefined) due to a use‐after‐free,
// while the fixed version correctly returns 42.
/////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate() {
        let value = calculate();
        assert_eq!(value, 42, "Test oracle: expected value to be 42");
    }
}
