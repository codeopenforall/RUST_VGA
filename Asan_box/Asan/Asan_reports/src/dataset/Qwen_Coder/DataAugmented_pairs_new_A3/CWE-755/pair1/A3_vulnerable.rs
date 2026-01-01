use std::thread;

struct Data;

impl Data {
    unsafe fn compute(_input: i32) -> Option<i32> {
        None
    }
}

fn run_contract(inputs: &[i32]) -> Result<(), &'static str> {
    for &input in inputs {
        let handle = thread::spawn(move || {
            let res = unsafe { Data::compute(input).unwrap_or_default() };
            if res < 0 {
                return Err("Negative result");
            }
            Ok(())
        });

        if let Err(_) = handle.join().unwrap() {
            return Err("Thread failed processing input");
        }
    }
    Ok(())
}

fn main() {
    let inputs = vec![-1, 5];
    let result = run_contract(&inputs);
    if result.is_ok() {
        println!("Unexpected success");
    } else {
        println!("Expected error: {}", result.unwrap_err());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exception_condition() {
        // Using a triggering input with a negative value.
        let inputs = vec![-1, 5];
        let result = run_contract(&inputs);
        // The contract expected for correct behavior is to return an error on negative input.
        // Therefore, if the function returns Ok, it indicates that the exception condition was swallowed.
        assert!(result.is_err(), "Expected error for negative input, got Ok: {:?}", result);
    }
}

// A standalone oracle function that can be used for external testing.
fn oracle() {
    let inputs = vec![-1, 5];
    let result = run_contract(&inputs);
    if result.is_ok() {
        panic!("Test oracle failed: expected error for negative input.");
    }
}
