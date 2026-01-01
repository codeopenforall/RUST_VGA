#![allow(unused)]
use std::sync::Arc;
use std::thread;
struct Processor {
    data: Vec<u32>,
}
impl Processor {
    fn new(data: Vec<u32>) -> Self {
        Processor { data }
    }
    fn process_at(&self, index: usize) -> Result<u32, &'static str> {
        unsafe {
            Ok(*self.data.as_ptr().add(index))
        }
    }
}
fn execute(index: usize) -> Result<u32, &'static str> {
    let proc = Processor::new(vec![10, 20, 30, 40]);
    let proc_arc = Arc::new(proc);
    let proc_clone = Arc::clone(&proc_arc);
    let handle = thread::spawn(move || {
        proc_clone.process_at(index)
    });
    handle.join().unwrap()
}
fn main() {
    let idx = 4;
    match execute(idx) {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::execute;

    // This test verifies the contract: when an out-of-bound index is used,
    // the correct error is returned in the fixed version.
    // In the vulnerable version, the function does not return an error,
    // so this test will fail.
    #[test]
    fn contract_test() {
        match execute(4) {
            Ok(val) => panic!("Contract violation: expected error for out-of-bound access, got value {}", val),
            Err(e) => assert_eq!(e, "Index out of bounds", "Unexpected error message"),
        }
    }
}
