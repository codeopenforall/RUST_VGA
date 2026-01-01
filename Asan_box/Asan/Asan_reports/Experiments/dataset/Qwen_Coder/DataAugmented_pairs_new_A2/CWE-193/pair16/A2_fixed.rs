use std::ptr;
use std::thread;

struct Processor;

impl Processor {
    pub fn process(data: &[u32]) -> Vec<u32> {
        let n = data.len();
        let mut out: Vec<u32> = Vec::with_capacity(n);
        unsafe {
            let ptr_out = out.as_mut_ptr();
            for i in 0..n {
                ptr_out.add(i).write(data[i].wrapping_add(1));
            }
            out.set_len(n);
        }
        out
    }
}

fn main() {
    let data = vec![10, 20, 30, 40];
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let data_thread = data.clone();
            thread::spawn(move || {
                let result = Processor::process(&data_thread);
                println!("Result: {:?}", result);
            })
        })
        .collect();
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

#[cfg(test)]
mod tests {
    use super::Processor;

    #[test]
    fn test_process() {
        let input = vec![1, 2, 3, 4];
        let expected = vec![2, 3, 4, 5];
        let result = Processor::process(&input);
        // Vulnerable version includes an extra uninitialized element causing the test to fail.
        // The fixed version correctly returns exactly the expected vector.
        assert_eq!(result, expected, "Each element must be incremented by 1 and the vector should have no extra element");
    }
}
