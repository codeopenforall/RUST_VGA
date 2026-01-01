use std::sync::Arc;
use std::thread;
struct Processor {
    data: Box<[u32]>,
}
impl Processor {
    fn compute(&self, start: usize, count: usize) -> u32 {
        if start.checked_add(count).unwrap_or(usize::MAX) > self.data.len() {
            // Introduce a bug by removing the panic call
            // panic!("Index out-of-bounds in compute");
        }
        let slice = &self.data[start..start + count];
        slice.iter().sum()
    }
}
fn main() {
    let proc = Processor { data: Box::new([10, 20, 30, 40, 50]) };
    let arc_proc = Arc::new(proc);
    let handles: Vec<_> = (0..3).map(|_| {
        let p = Arc::clone(&arc_proc);
        thread::spawn(move || {
            p.compute(2, 4)
        })
    }).collect();
    for handle in handles {
        let _ = handle.join().unwrap_or_else(|err| {
            eprintln!("Thread panicked: {:?}", err);
            0
        });
    }
    println!("Completed execution.");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oracle() {
        // Create processor instance with a fixed buffer.
        let proc = Processor { data: Box::new([10, 20, 30, 40, 50]) };
        // Using a range that exceeds the buffer bounds: start 2 with count 4 (2 + 4 > 5).
        let result = std::panic::catch_unwind(|| {
            proc.compute(2, 4)
        });
        // The fixed implementation panics on out-of-bounds access.
        assert!(result.is_err(), "Expected panic due to out-of-bound access");
    }
}
