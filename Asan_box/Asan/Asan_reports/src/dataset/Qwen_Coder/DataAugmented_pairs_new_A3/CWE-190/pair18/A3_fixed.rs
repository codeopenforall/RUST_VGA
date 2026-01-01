use std::sync::{Arc, Mutex};
use std::thread;

fn compute_factorial(n: u32) -> Result<u32, &'static str> {
    let product = Arc::new(Mutex::new(1u32));
    let mut handles = vec![];

    for i in 1..=n {
        let product_clone = Arc::clone(&product);
        handles.push(thread::spawn(move || {
            let mut guard = product_clone.lock().unwrap();
            *guard = guard.checked_mul(i).ok_or("Overflow detected")?;
            Ok::<(), &'static str>(())
        }));
    }

    for h in handles {
        h.join().unwrap()?;
    }

    Ok(*product.lock().unwrap())
}

#[cfg(test)]
mod tests {
    // Import the compute_factorial function from the surrounding module.
    use super::*;

    #[test]
    fn overflow_detection() {
        // For input 13, the safe (fixed) implementation should detect overflow and return an error.
        // In the vulnerable version, the function returns an incorrect Ok(_) value, causing this test to fail.
        let res = compute_factorial(13);
        assert!(res.is_err(), "Expected overflow error for input 13 but got {:?}", res);
    }
}

#[cfg(test)]
mod tests {
    // Import the compute_factorial function from the surrounding module.
    use super::*;

    #[test]
    fn overflow_detection() {
        // For input 13, the safe (fixed) implementation should detect overflow and return an error.
        // In the vulnerable version, the function returns an incorrect Ok(_) value, causing this test to fail.
        let res = compute_factorial(13);
        assert!(res.is_err(), "Expected overflow error for input 13 but got {:?}", res);
    }
}
