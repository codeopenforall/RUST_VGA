use std::thread;

struct Processor;

impl Processor {
    pub fn compute(input: u32) -> Result<u64, &'static str> {
        let capacity: u32 = unsafe {
            input.wrapping_mul(1000)
        };
        let handle = thread::spawn(move || {
            let cap_u64 = capacity as u64;
            let sum = (cap_u64 * cap_u64.wrapping_sub(1)) / 2;
            sum
        });
        match handle.join() {
            Ok(result) => Ok(result),
            Err(_) => Err("Thread panicked"),
        }
    }
}

fn main() {
    let input: u32 = 5_000_000;
    match Processor::compute(input) {
        Ok(result) => {
            println!("Computed result: {}", result);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

/*
   The test oracle function uses a deterministic input that triggers an integer overflow in the
   vulnerable code. It asserts that calling the compute method returns an error.
   For the vulnerable version, the unchecked wrapping arithmetic causes an incorrect Ok result,
   causing the oracle to fail the test; whereas the fixed version returns an Err which satisfies the test.
*/
#[cfg(test)]
mod tests {
    use super::Processor;

    #[test]
    fn test_overflow_detection() {
        // Use an input that will cause input * 1000 to overflow a u32.
        let input: u32 = 5_000_000;
        let result = Processor::compute(input);
        // The correct behavior is to detect the overflow and return an error.
        assert!(result.is_err(), "Overflow was not detected as expected");
    }
}
  
// For running the test oracle separately.
#[allow(dead_code)]
fn test_oracle() {
    tests::test_overflow_detection();
}
