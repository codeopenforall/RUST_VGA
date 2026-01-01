use std::sync::{Arc, Mutex};
use std::thread;

struct Guard {
    balance: u32,
}

impl Guard {
    fn new(balance: u32) -> Self {
        Guard { balance }
    }

    fn add(&mut self, amount: u32) -> Result<(), &'static str> {
        self.balance = self.balance.checked_add(amount).ok_or("Overflow occurred")?;
        Ok(())
    }

    fn current(&self) -> u32 {
        self.balance
    }
}

fn run_calculation(init: u32, add: u32, threads: usize) -> Result<u32, &'static str> {
    let guard = Arc::new(Mutex::new(Guard::new(init)));
    let mut handles = vec![];

    for _ in 0..threads {
        let guard_clone = Arc::clone(&guard);
        let handle = thread::spawn(move || -> Result<(), &'static str> {
            let mut guard = guard_clone.lock().unwrap();
            guard.add(add)
        });
        handles.push(handle);
    }

    for handle in handles {
        let res = handle.join().map_err(|_| "Thread panicked")?;
        if let Err(e) = res {
            return Err(e);
        }
    }

    let guard = guard.lock().unwrap();
    Ok(guard.current())
}

fn try_compute() -> Result<u32, &'static str> {
    run_calculation(u32::MAX - 10, 15, 1)
}

#[cfg(test)]
mod tests {
    use super::try_compute;

    #[test]
    fn test_overflow_detection() {
        match try_compute() {
            Ok(val) => {
                panic!("Test failed: expected error but got value {}", val);
            },
            Err(e) => {
                assert_eq!(e, "Overflow occurred");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::try_compute;

    // This test acts as an oracle:
    // - For the vulnerable variant, try_compute() returns Ok(result) even when overflow occurs.
    //   The expected behavior is to detect overflow and return an error.
    //   Therefore, the test must fail for the vulnerable code.
    // - For the corrected variant, try_compute() returns Err("Overflow occurred"),
    //   and that is the expected behavior.
    #[test]
    fn test_overflow_detection() {
        match try_compute() {
            Ok(val) => {
                // In the vulnerable version, the unchecked addition causes overflow wraparound.
                // For the specific parameters (u32::MAX - 10 plus 15), the mathematically correct 
                // result would overflow; hence, receiving a numeric value means the overflow was not detected.
                panic!("Test failed: expected error but got value {}", val);
            },
            Err(e) => {
                assert_eq!(e, "Overflow occurred");
            }
        }
    }
}
