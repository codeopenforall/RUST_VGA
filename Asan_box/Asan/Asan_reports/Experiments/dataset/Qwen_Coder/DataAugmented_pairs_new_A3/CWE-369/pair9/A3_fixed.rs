use std::thread;

pub fn compute(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}

fn main() {
    let res = compute(50, 0);
    match res {
        Ok(val) => println!("Computation result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
    let handle = thread::spawn(|| {
        let res_thread = compute(100, 0);
        match res_thread {
            Ok(val) => println!("Thread result: {}", val),
            Err(e) => println!("Thread error: {}", e),
        }
    });
    let _ = handle.join();
}

///////////////////////////////
// Test Oracle Function
///////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;
    // Import the compute function from the current module.
    use super::compute;

    #[test]
    fn test_oracle() {
        // In a safe implementation the call with a zero denominator should return an Err without panicking.
        let result = panic::catch_unwind(|| {
            compute(100, 0)
        });
        match result {
            Ok(res) => {
                // The fixed version returns an error variant.
                assert!(res.is_err(), "Expected an Err for a division by zero case");
            },
            Err(_) => {
                panic!("Test failed: unexpected panic due to division by zero vulnerability");
            }
        }
    }
}
