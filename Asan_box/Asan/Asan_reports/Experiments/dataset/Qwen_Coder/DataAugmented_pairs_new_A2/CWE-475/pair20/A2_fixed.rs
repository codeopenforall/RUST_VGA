use std::thread;

struct Container {
    payload: Vec<u8>,
}

impl Container {
    fn compute(&self) -> Option<u32> {
        if self.payload.len() < 12 {
            return None;
        }
        let mut sum = 0u32;
        unsafe {
            let ptr = self.payload.as_ptr() as *const u32;
            for i in 0..3 {
                sum = sum.wrapping_add(ptr.add(i).read());
            }
        }
        Some(sum)
    }
}

fn run() {
    let data: Vec<u8> = vec![
        1, 0, 0, 0,   
        2, 0, 0, 0,   
        3, 0, 0, 0    
    ];
    let container = Container { payload: data };
    let handle = thread::spawn(move || {
        container.compute()
    });
    match handle.join().unwrap() {
        Some(result) => println!("Computed sum: {}", result),
        None => println!("Invalid input length")
    }
}

fn main() {
    run();
}

#[cfg(test)]
mod tests {
    use std::thread;
    // Import the Container from the current crate.
    use crate::Container;

    // This test serves as the oracle for the contract.
    // It uses an input that is intentionally too short (8 bytes) so that:
    // - In the corrected version, the compute method returns None.
    // - In the vulnerable version, the lack of precondition checking leads to undefined behavior,
    //   typically resulting in an incorrect (non-None) value or even a panic.
    #[test]
    fn test_container() {
        let container = Container { payload: vec![1u8, 2, 3, 4, 5, 6, 7, 8] };

        let handle = thread::spawn(move || {
            container.compute()
        });

        let result = handle.join().expect("Thread panicked");
        // The contract expects that an invalid input returns None.
        assert_eq!(result, None, "Expected None due to invalid input; computation did not fail as expected");
    }
}
