use std::thread;
struct Data {
    value: u32,
}
fn compute() -> u32 {
    let resource = Box::new(Data { value: 100 });
    let ptr = Box::into_raw(resource);
    unsafe {
        let _ = Box::from_raw(ptr);
    }
    let handle = thread::spawn(|| {
        let _ = Box::new(Data { value: 200 });
    });
    handle.join().unwrap();
    unsafe { (*ptr).value }
}
fn main() {
    let result = compute();
    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_oracle() {
        // The contract is that the compute() function must return exactly 100.
        // In the vulnerable version, the use-after-free may lead to an incorrect value.
        // In the fixed version, the function returns the expected value.
        assert_eq!(compute(), 100, "The computed result must be 100");
    }
}
