use std::sync::atomic::{AtomicI32, Ordering};

const THRESHOLD: i32 = 1000;

static SHARED: AtomicI32 = AtomicI32::new(0);

pub fn run_app(d: i32) {
    loop {
        let current = SHARED.load(Ordering::SeqCst);
        if current + d >= THRESHOLD {
            break;
        }
        if SHARED.compare_exchange(current, current + d, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            break;
        }
    }
    let final_val = SHARED.load(Ordering::SeqCst);
    assert!(final_val < THRESHOLD, "Safe limit exceeded unexpectedly!");
    println!("Completed run with SHARED value: {}", final_val);
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
