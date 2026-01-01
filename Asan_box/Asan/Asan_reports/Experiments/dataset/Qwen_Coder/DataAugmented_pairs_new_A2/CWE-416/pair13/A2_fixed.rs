use std::sync::Arc;
use std::thread;

#[derive(Debug)]
pub struct Data {
    pub value: i32,
}

pub fn process() -> i32 {
    let data = Arc::new(Data { value: 512 });
    let data_clone = Arc::clone(&data);
    data_clone.value
}

fn main() {
    let result = process();
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        // Expect that the function returns 512.
        // In the vulnerable version, undefined behavior may cause the test to fail.
        // In the corrected version, proper memory management ensures the returned value is correct.
        let result = process();
        assert_eq!(result, 512, "The result should correctly be 512.");
    }
}
