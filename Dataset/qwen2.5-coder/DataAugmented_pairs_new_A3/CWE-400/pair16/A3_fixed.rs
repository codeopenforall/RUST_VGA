use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
use std::thread;

const MAX_LIMIT: u32 = 1000;

fn process(limit: u32) -> Result<(), &'static str> {
    if limit > MAX_LIMIT {
        return Err("Limit exceeds safe threshold");
    }

    let total = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];

    for i in 0..limit {
        let total_clone = total.clone();
        let handle = thread::spawn(move || {
            let local = i as u64;
            let local = local.wrapping_add(1);
            total_clone.fetch_add(local, Ordering::Relaxed);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_oracle() {
        let result = process(1500);
        assert!(result.is_err(), "Expected an error when the limit exceeds safe threshold");
    }
}