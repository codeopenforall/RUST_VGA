use std::thread;
#[derive(Debug, PartialEq)]
enum OperationStatus {
    Success,
    Failure,
    Timeout,
}
struct Processor;
impl Processor {
    fn perform(&self, input: &str) -> Result<i32, &'static str> {
        let input_owned = input.to_owned();
        let handler = thread::spawn(move || {
            unsafe {
                let dummy_ptr: *const i32 = &10;
                let _ = *dummy_ptr; 
            }
            if input_owned == "fail" {
                OperationStatus::Failure
            } else if input_owned == "timeout" {
                OperationStatus::Timeout
            } else {
                OperationStatus::Success
            }
        });
        let op_status = handler.join().map_err(|_| "Thread panicked")?;
        match op_status {
            OperationStatus::Success => Ok(42),
            OperationStatus::Failure => Err("Operation failed"),
            OperationStatus::Timeout => Err("Operation timed out"),
        }
    }
}
fn main() {
    let proc = Processor;
    let res = proc.perform("test").expect("Expected operation to succeed");
    println!("Result: {}", res);
}

//////////////////////////////
// Test Oracle Function     //
// This test asserts that when "fail" is passed as input,
// the vulnerable implementation incorrectly returns success,
// whereas the fixed implementation properly returns an error.
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::Processor;

    // The test oracle function should be run against both implementations.
    // For the vulnerable version, this test is expected to fail since "fail" is misinterpreted.
    // For the fixed version, it should pass by returning an Err("Operation failed").
    #[test]
    fn oracle_test() {
        let proc = Processor;
        let result = proc.perform("fail");
        // The expected behavior is to get an error with a specific message.
        match result {
            Ok(_) => panic!("Test failed: Expected an error on input \"fail\""),
            Err(e) => assert_eq!(e, "Operation failed", "Unexpected error message"),
        }
    }
}
