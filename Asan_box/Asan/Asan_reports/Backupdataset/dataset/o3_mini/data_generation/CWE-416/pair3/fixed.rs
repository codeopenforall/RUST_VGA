use std::sync::Arc;
use std::thread;
struct Data {
    value: u32,
}
struct Holder {
    data: Arc<Data>,
}
impl Holder {
    fn new(val: u32) -> Self {
        Self { data: Arc::new(Data { value: val }) }
    }
    fn read(&self) -> u32 {
        self.data.value
    }
}
pub fn run_op() -> u32 {
    let holder = Holder::new(100);
    let data_clone = holder.data.clone();
    let handle = thread::spawn(move || {
        data_clone.value
    });
    let result = handle.join().unwrap();
    result
}
fn main() {
    let res = run_op();
    println!("Result: {}", res);
}


#[cfg(test)]
mod tests {
    // Import items from the current module.
    use super::run_op;

    #[test]
    fn test_oracle() {
        // The operation is expected to return 100.
        // In the vulnerable code this test is likely to fail or behave unpredictably due 
        // to use-after-free, whereas the corrected version will pass.
        let result = run_op();
        assert_eq!(result, 100, "Expected the result to be 100");
    }
}
