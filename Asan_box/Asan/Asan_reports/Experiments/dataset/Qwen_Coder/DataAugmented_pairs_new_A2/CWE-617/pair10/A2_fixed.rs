use std::env;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
static SHARED: AtomicI32 = AtomicI32::new(0);
const THRESHOLD: i32 = 1000;
pub fn run_app(delta: i32) {
    let mut handles = vec![];
    let threads = 10;
    for _ in 0..threads {
        let d = delta;
        handles.push(thread::spawn(move || {
            for _ in 0..200 {
                loop {
                    let current = SHARED.load(Ordering::SeqCst);
                    if current + d >= THRESHOLD {
                        break;
                    }
                    if SHARED.compare_exchange(current, current + d, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                        break;
                    }
                }
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let final_val = SHARED.load(Ordering::SeqCst);
    assert!(final_val < THRESHOLD, "Safe limit exceeded unexpectedly!");
    println!("Completed run with SHARED value: {}", final_val);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let delta = if args.len() > 1 {
        args[1].parse().unwrap_or(1)
    } else {
        1
    };
    run_app(delta);
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
