use std::sync::{Arc, Mutex};
use std::thread;
pub struct Executor {
    pub special: bool,
    pub value: i32,
}
impl Executor {
    pub fn process_input(&mut self, input: i32) -> i32 {
        unsafe {
            let p = &mut self.value as *mut i32;
            match self.special {
                true => {
                    *p = input * 2; // Vulnerable: Incorrect operation for special == true
                },
                false => {
                    *p = input + 50;
                }
            }
            *p
        }
    }
}
fn main() {
    let executor = Arc::new(Mutex::new(Executor { special: true, value: 0 }));
    let mut handles = Vec::new();
    for i in 1..=4 {
        let exec_clone = Arc::clone(&executor);
        handles.push(thread::spawn(move || {
            let mut guard = exec_clone.lock().unwrap();
            guard.process_input(i)
        }));
    }
    for handle in handles {
        let _ = handle.join().unwrap();
    }
    let final_val = executor.lock().unwrap().value;
    println!("Final value: {}", final_val);
}

////////////////////////////
// Test Oracle Function Code
////////////////////////////

#[cfg(test)]
mod tests {
    use super::Executor;

    // The test instantiates an executor in special mode (special == true)
    // and processes a controlled input (4). In the correct implementation,
    // the executor should add 50 to the input, yielding 54.
    // In the vulnerable version, the branch error produces 4 * 2 = 8.
    // This test will fail against the vulnerable code and pass when the issue is fixed.
    #[test]
    fn contract_executor() {
        let mut exec = Executor { special: true, value: 0 };
        let result = exec.process_input(4);
        assert_eq!(
            result, 54,
            "Contract violation: expected result 54, got {}",
            result
        );
    }
}
