#![allow(unused)]
struct Data {
    value: i32,
}
impl Data {
    fn new(v: i32) -> Self {
        Data { value: v }
    }
}
pub fn run() -> i32 {
    let resource = Box::new(Data::new(10));
    let raw_ptr = Box::into_raw(resource);
    unsafe {
        let box_a = Box::from_raw(raw_ptr);
        box_a.value * 2
    }
}
fn main() {
    println!("Result: {}", run());
}


#[cfg(test)]
mod tests {
    use super::run;
    use std::panic;

    #[test]
    fn test_oracle() {
        // The test expects a successful execution (i.e. no panic).
        // In the vulnerable implementation, a double free triggers a panic.
        let result = panic::catch_unwind(|| run());
        assert!(result.is_ok(), "The function panicked, indicating a double free vulnerability");
        // Also verify the computed result.
        let value = result.expect("Expected a successful result");
        assert_eq!(value, 20, "Unexpected computation result");
    }
}
