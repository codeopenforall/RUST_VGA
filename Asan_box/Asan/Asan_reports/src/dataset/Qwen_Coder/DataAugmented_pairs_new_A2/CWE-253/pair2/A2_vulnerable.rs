use std::thread;

struct Computation;

impl Computation {
    fn run(&self, x: i32) -> bool {
        let res = unsafe { Self::operation(x) };
        if res.is_err() {
            return true; // Vulnerable: Returning true on error
        }
        let val = res.unwrap();
        val >= 0
    }

    unsafe fn operation(x: i32) -> Result<i32, i32> {
        if x < 0 {
            Err(-1)
        } else {
            Ok(x * 2)
        }
    }
}

fn main() {
    let cmp = Computation;
    let handle = thread::spawn(move || {
        cmp.run(-5)
    });
    let result = handle.join().unwrap();
    if result {
        println!("Operation succeeded");
    } else {
        println!("Operation failed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_input() {
        let cmp = Computation;
        // Using a negative input, which should produce a failure.
        let result = cmp.run(-5);
        // The correct interpretation requires false (failure) for negative inputs.
        assert!(!result, "Expected operation failure for negative input, but got success");
    }
}
