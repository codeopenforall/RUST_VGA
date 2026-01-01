use std::thread;
use std::time::Duration;
#[derive(Debug)]
pub struct Data {
    pub value: i32,
}
pub fn process() -> i32 {
    let ptr: *mut Data;
    unsafe {
        let boxed = Box::new(Data { value: 512 });
        ptr = Box::into_raw(boxed);
        Box::from_raw(ptr);
        thread::sleep(Duration::from_millis(100));
        (*ptr).value
    }
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
