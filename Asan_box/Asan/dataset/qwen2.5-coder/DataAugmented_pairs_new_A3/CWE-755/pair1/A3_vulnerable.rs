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