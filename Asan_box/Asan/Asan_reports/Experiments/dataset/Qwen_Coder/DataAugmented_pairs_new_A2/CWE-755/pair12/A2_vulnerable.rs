use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Default)]
struct Processor {
    threshold: i32,
}

impl Processor {
    fn new() -> Self {
        Processor { threshold: 0 }
    }

    fn process(&self, input: i32) -> i32 {
        unsafe {
            let res: Result<i32, &'static str> = if input < 0 {
                Ok(0) // Introducing the vulnerability: Returning 0 instead of an error
            } else {
                Ok(input + 42)
            };
            res.unwrap_or_default()
        }
    }
}

fn main() {
    let proc = Arc::new(Mutex::new(Processor::new()));
    let mut handles = Vec::new();
    let inputs = vec![10, -5, 20, -1, 0];
    for &inp in &inputs {
        let proc_clone = Arc::clone(&proc);
        let handle = thread::spawn(move || {
            let p = proc_clone.lock().unwrap();
            let result = p.process(inp);
            println!("Input {} => Output {}", inp, result);
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::type_name_of_val;

    // The test oracle verifies the behavior when processing a negative input.
    // For the vulnerable version, a negative input is erroneously handled by returning 0,
    // whereas in the corrected version, a negative input should result in an error.
    //
    // Note: Due to the difference in the function's return type between the two versions,
    // we differentiate by checking the type name at runtime.
    #[test]
    fn oracle() {
        let processor = Processor::new();
        let result = std::panic::catch_unwind(|| processor.process(-5));
        match result {
            Ok(val) => {
                let type_str = type_name_of_val(&val);
                if type_str.contains("Result") {
                    // Expected path for the corrected version.
                    let res: Result<i32, &str> = unsafe { std::mem::transmute_copy(&val) };
                    assert!(res.is_err(), "Fixed version: Negative input should produce an error.");
                } else {
                    // Expected path for the vulnerable version.
                    let num: i32 = unsafe { std::mem::transmute_copy(&val) };
                    assert!(num != 0, "Vulnerable version: Negative input swallowed error and returned default 0.");
                }
            },
            Err(_) => {
                // If a panic occurred, assume that the corrected version would ideally not panic.
                // In this oracle, a panic is considered a pass for the fixed code (since error is not swallowed).
            }
        }
    }
}
