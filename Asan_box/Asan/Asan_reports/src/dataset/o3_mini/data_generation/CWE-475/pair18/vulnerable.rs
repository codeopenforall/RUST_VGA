use std::slice;
trait Processor {
    fn run(&self, data: &[u8]) -> u32;
}
struct Calculator;
impl Processor for Calculator {
    fn run(&self, data: &[u8]) -> u32 {
        unsafe {
            let ptr = data.as_ptr().offset(1);
            let sub_slice = slice::from_raw_parts(ptr, data.len() - 1);
            sub_slice.iter().fold(0, |acc, &x| acc + x as u32)
        }
    }
}
fn main() {
    let input = std::env::args().nth(1).unwrap_or_default();
    let bytes = input.as_bytes();
    let calc = Calculator;
    let res = calc.run(bytes);
    println!("{}", res);
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This test uses an empty byte slice as triggering input.
    // For the vulnerable version, the unchecked pointer arithmetic should
    // lead to undefined behavior (and typically a panic in debug builds),
    // while the fixed version safely returns 0.
    #[test]
    fn test_empty_input_handling() {
        let input = b"";
        let calc = Calculator;
        let result = panic::catch_unwind(|| calc.run(input));
        match result {
            // In the fixed version, no panic occurs and the function returns 0.
            Ok(val) => {
                assert_eq!(val, 0, "Expected a 0 return value for empty input in the safe implementation");
            }
            // In the vulnerable build, the function is expected to panic due to undefined behavior.
            Err(_) => {
                panic!("Function panicked for empty input, indicating unsafe handling of API preconditions");
            }
        }
    }
}
