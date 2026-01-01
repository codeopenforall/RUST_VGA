use std::boxed::Box;
struct Info {
    value: i32,
}
impl Info {
    pub fn crunch() -> i32 {
        let original = Box::new(Info { value: 42 });
        let ret = original.value;
        ret
    }
}
fn main() {
    let res = Info::crunch();
    println!("Result: {}", res);
}


//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::Info;
    
    // This test asserts that the function returns the expected value.
    // For the vulnerable version, due to use-after-free, the result may be unpredictable
    // and the test is expected to fail (or exhibit undefined behavior).
    // In the safe version, the expected value 42 is returned reliably.
    #[test]
    fn test_oracle() {
        let result = Info::crunch();
        assert_eq!(result, 42, "Expected value 42; use-after-free vulnerability may cause deviation.");
    }
}
