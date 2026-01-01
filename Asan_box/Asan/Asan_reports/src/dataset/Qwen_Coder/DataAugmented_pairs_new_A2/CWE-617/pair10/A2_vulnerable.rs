use std::env;
use std::thread;
static mut GLOBAL_VAR: i32 = 0;
const LIMIT: i32 = 1000;
pub fn run_app(increment: i32) {
    let mut threads = vec![];
    let num_threads = 10;
    for _ in 0..num_threads {
        let inc = increment;
        threads.push(thread::spawn(move || {
            for _ in 0..200 {
                unsafe {
                    GLOBAL_VAR = GLOBAL_VAR.wrapping_add(inc);
                }
            }
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    unsafe {
        assert!(GLOBAL_VAR < LIMIT, "Global variable exceeded safe limit!");
        println!("Completed run with GLOBAL_VAR value: {}", GLOBAL_VAR);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let increment = if args.len() > 1 {
        args[1].parse().unwrap_or(1)
    } else {
        1
    };
    run_app(increment);
}

#[cfg(test)]
mod tests {
    use std::panic;
    
    // This test oracle calls the application's run function with an attacker-controlled input.
    // For the vulnerable build (compiled with `--features vulnerable`), the unsynchronized update
    // leads to the THRESHOLD being exceeded and the reachable assertion triggering a panic.
    // For the safe build, the atomic update prevents overshooting the threshold, and no panic occurs.
    #[test]
    fn oracle() {
        // An attacker-supplied delta value that will force the shared value to exceed the threshold in the unsafe version.
        let attacker_input = 100;
    
        let result = panic::catch_unwind(|| {
            // Assumes that run_app is exposed from the current crate.
            crate::run_app(attacker_input);
        });
    
        #[cfg(feature = "vulnerable")]
        {
            assert!(result.is_err(), "Test oracle: Expected panic due to unsafe state update.");
        }
    
        #[cfg(not(feature = "vulnerable"))]
        {
            assert!(result.is_ok(), "Test oracle: Unexpected panic in safe update.");
        }
    }
}
