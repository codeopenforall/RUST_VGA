use std::env;
use std::thread;

struct Math;
impl Math {
    fn calc(&self, x: i32) -> Option<i32> {
        if x == 0 {
            None
        } else {
            Some(unsafe { 100 / x })
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: i32 = if args.len() > 1 {
        args[1].parse().unwrap_or(1)
    } else {
        1
    };
    let math = Math;
    let handle = thread::spawn(move || {
        let res = math.calc(input);
        println!("Outcome: {:?}", res);
    });
    handle.join().unwrap();
}

/* 
   Test Oracle:
   This test function verifies that calling the calculation function with zero as the input 
   does not cause a panic. It uses std::panic::catch_unwind to capture any unexpected panic.
   In the safe revision, invoking the calculation with a zero divisor should return None.
   In the vulnerable version the same call is expected to panic, leading this test to fail.
*/
#[cfg(test)]
mod tests {
    use super::Math;
    use std::panic;

    #[test]
    fn test_division_zero() {
        let math = Math;
        let outcome = panic::catch_unwind(|| math.calc(0));
        match outcome {
            Err(_) => panic!("Test failed: unexpected panic on zero divisor."),
            Ok(res) => {
                assert!(res.is_none(), "Test failed: Expected None for zero divisor, got {:?}", res);
            }
        }
    }
}
